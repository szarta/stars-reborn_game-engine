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
use configuration;
use ::game::objects::game::Game;

pub fn store_game(game : &Game) {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let v = serde_json::to_string(&game).unwrap();

    let mut statement = connection
        .prepare("INSERT INTO games (gid, year, contents) VALUES (?,?,?)")
        .unwrap();

    let game_id = game.id.to_string();

    statement.bind(1, &sqlite::Value::String(game_id)).unwrap();
    statement.bind(2, &sqlite::Value::Integer(game.year as i64)).unwrap();
    statement.bind(3, &sqlite::Value::String(v)).unwrap();
    statement.next().unwrap();
}

pub fn create_db_if_necessary() {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS games (gid TEXT, year INTEGER, contents TEXT)
        "
    ).unwrap();
}
