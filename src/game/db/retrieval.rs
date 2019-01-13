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
use serde_json;
use sqlite::State;

use configuration;

use ::game::objects::game::Game;

pub fn lookup_game(game_id: &str, game_year: u32) -> Option<Game> {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM games WHERE (gid = ?) AND (year = ?) LIMIT 1")
        .unwrap();

    c.bind(1, &sqlite::Value::String(game_id.to_string())).unwrap();
    c.bind(2, &sqlite::Value::Integer(game_year as i64)).unwrap();

    while let State::Row = c.next().unwrap() {
        let buf : String = c.read::<String>(2).unwrap();
        println!("{}", buf);
        let g = serde_json::from_str(&buf).unwrap();
        return Some(g);
    }

    return None;
}
