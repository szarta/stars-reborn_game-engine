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
extern crate dotenv;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate sqlite;

extern crate argparse;
extern crate uuid;
extern crate rand;
extern crate flate2;

extern crate hyper;
extern crate futures;
extern crate url;
extern crate valico;

#[macro_use]
extern crate lazy_static;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use argparse::{ArgumentParser, Print};

pub mod service;
pub mod schemas {
    pub mod create_tutorial_game;
}

pub mod configuration;
pub mod game {
    pub mod objects {
        pub mod game;
        pub mod universe;
        pub mod planet;
        pub mod player;
        pub mod tech;
        pub mod race;
        pub mod fleet;
        pub mod predefined {
            pub mod races;
            pub mod messages;
            pub mod planets;
            pub mod fleets;
        }
    }
    pub mod tutorial;
    pub mod db {
        pub mod storage;
        pub mod retrieval;
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Runs the game engine interface for stars-reborn");
        ap.add_option(&["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");

        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                std::process::exit(x);
            }
        }
    }

    ::game::db::storage::create_db_if_necessary();

    /*
    let u = ::game::tutorial::setup::generate_tutorial_universe();
    let size = ::game::objects::universe::UniverseSize::Huge;
    let density = ::game::objects::universe::UniverseDensity::Packed;
    let clumping = false;
    let players = Vec::new();
    let starting_distance = ::game::objects::game::PlayerStartingDistance::Close;
    let u = ::game::objects::universe::Universe::construct_random(&size, &density, &clumping, &players, &starting_distance);
    let j = serde_json::to_string(&u).unwrap();
    info!("universe bytes: {}", j.len());
    debug!("{}", j);
    */

    let server_addr = configuration::get_server_ip().parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&server_addr, || Ok(service::GameService {}))
        .unwrap();

    info!("Running server on {}", server_addr);
    server.run().unwrap();
}
