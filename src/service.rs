/*
 *  Copyright 2018 Brandon Arrendondo
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 *  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 *  DEALINGS IN THE SOFTWARE.
 */
use hyper;
use hyper::server::{Request, Response, Service};
use hyper::{StatusCode};
use hyper::Method::{Get, Post};
use hyper::header::{ContentLength, ContentType};

use url;

use serde_json;
use valico::json_schema;

use futures;
use futures::Stream;
use futures::future::{Future};

use std::collections::HashMap;

use ::game::objects::planet::PlanetShortSummary;

pub struct GameService {
}

lazy_static! {
static ref SCHEMAS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("/new_tutorial_game", ::schemas::create_tutorial_game::CREATE_TUTORIAL_GAME_SCHEMA);
        m
    };
}

fn json_request_is_valid(payload: hyper::Chunk, api: &str) -> bool {
    match serde_json::from_slice(&payload) {
        Ok(json_payload) => {
            let mut scope = json_schema::Scope::new();
            let schema = SCHEMAS.get(&api).unwrap().clone();
            let schema = serde_json::from_str(schema).unwrap();
            let compiled_schema = scope.compile_and_return(schema, false).unwrap();
            compiled_schema.validate(&json_payload).is_valid()
        }
        Err(_e) => {
            false
        }
    }
}

fn json_build_invalid_request_response() -> Response {
    let payload = json!({
        "request-is-valid": false
    }).to_string();

    Response::new()
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload)
}

fn get_turn_response(query: &str) -> Response {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    /*
    args.get("game_id")
    args.get("player_id")
    */

    let payload = json!({
        "request-is-valid": true
    }).to_string();

    Response::new()
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload)

}

#[derive(Serialize, Deserialize)]
pub struct PlanetDump {
    pub request_is_valid : bool,
    pub planets : Vec<PlanetShortSummary>
}

fn get_planet_dump(query: &str) -> Response {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    match args.get("game_id") {
        Some(game_id) => {

            match ::game::db::retrieval::lookup_game(game_id, ::game::objects::game::STARTING_YEAR) {
                Some(game) => {
                    let mut dump : PlanetDump = PlanetDump {
                        request_is_valid: true,
                        planets: Vec::new()
                    };

                    for planet in game.universe.planets.iter() {
                        debug!("p: {}:{}:{}:{}", planet.id, planet.location.x, planet.location.y, planet.name);
                        dump.planets.push(PlanetShortSummary::construct_from_planet(&planet));
                    }

                    let payload = serde_json::to_string(&dump).unwrap();

                    Response::new()
                        .with_header(ContentLength(payload.len() as u64))
                        .with_header(ContentType::json())
                        .with_body(payload)
                
                }
                None => {
                    let payload = json!({
                        "request-is-valid": true
                    }).to_string();

                    Response::new()
                        .with_header(ContentLength(payload.len() as u64))
                        .with_header(ContentType::json())
                        .with_body(payload)
                }
            }
        }
        None => {
            json_build_invalid_request_response()
        }
    }
}



impl Service for GameService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("Received request: {:?}", request);

        match (request.method(), request.path()) {
            (&Get, "/version") => {
                let payload = json!({
                    "version": env!("CARGO_PKG_VERSION").to_string()
                }).to_string();

                let response = Response::new()
                    .with_header(ContentLength(payload.len() as u64))
                    .with_header(ContentType::json())
                    .with_body(payload);

                Box::new(futures::future::ok(response))
            },
            (&Get, "/turn") => {
                let response = match request.query() {
                    Some(query) => {
                        futures::future::ok(get_turn_response(query))
                    }
                    None => {
                        futures::future::ok(json_build_invalid_request_response())
                    }
                };

                Box::new(response)
            },
            (&Get, "/planet_dump") => {
                let response = match request.query() {
                    Some(query) => {
                        futures::future::ok(get_planet_dump(query))
                    }
                    None => {
                        futures::future::ok(json_build_invalid_request_response())
                    }
                };

                Box::new(response)
            },
            (&Post, "/new_tutorial_game") => {
                let future = request.body().concat2().and_then( |body| {
                    if json_request_is_valid(body, "/new_tutorial_game") {
                        let generated_game = ::game::tutorial::generate_tutorial_game();
                        // generate turn 0
                        let game_id = generated_game.id.to_string();
                        ::game::db::storage::store_game(&generated_game);

                        let payload = json!({
                            "request-is-valid": true,
                            "game-id": game_id
                        }).to_string();

                        let response = Response::new()
                            .with_header(ContentLength(payload.len() as u64))
                            .with_header(ContentType::json())
                            .with_body(payload);

                         futures::future::ok(response)
                    }
                    else {
                        futures::future::ok(json_build_invalid_request_response())
                   }
                });
                Box::new(future)
            },
            _=> Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
                )),
        }
    }
}
