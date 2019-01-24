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
use ::game::objects::universe::UniverseDensity;
use ::game::objects::universe::UniverseSize;
use ::game::objects::universe::Universe;
use ::game::objects::player::Player;
use uuid::Uuid;

pub const STARTING_YEAR : u32 = 2400;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub year: u32,
    pub parameters: GameParameters,
    pub universe: Universe
}

#[derive(Serialize, Deserialize)]
pub struct VictoryConditions {
    pub owns_percent_planets: bool,
    pub percent_planets: u8,

    pub obtains_tech_levels: bool,
    pub required_tech_level: u8,
    pub number_tech_fields: u8,

    pub exceeds_score: bool,
    pub score_to_exceed: u32,

    pub exceeds_second_place_score: bool,
    pub exceeds_second_place_by: u32,

    pub has_production_capacity: bool,
    pub minimum_production_capacity: u32,

    pub owns_capital_ships: bool,
    pub minimum_capital_ships: u32,

    pub has_highest_score: bool,
    pub highest_score_years: u32,

    pub number_of_criteria: u8,
    pub minimum_years: u32
}

#[derive(Serialize, Deserialize)]
pub struct GameParameters {
    pub allow_random_events: bool,
    pub accelerated_play: bool,
    pub public_player_scores: bool,
    pub maximum_minerals: bool,
    pub slow_tech_advances: bool,
    pub galaxy_clumping: bool,
    pub player_starting_distance: PlayerStartingDistance,
    pub universe_size: UniverseSize,
    pub universe_density: UniverseDensity,
    pub victory_conditions: VictoryConditions
}

#[derive(Serialize, Deserialize)]
pub enum PlayerStartingDistance {
    Close,
    Moderate,
    Farther,
    Distant
}

impl Game {
    pub fn construct(name: String, players: Vec<Player>, parameters: GameParameters) -> Game {
        let gid = Uuid::new_v4().to_string();
        let u = Universe::construct_random(&parameters.universe_size, &parameters.universe_density, &parameters.galaxy_clumping, &parameters.player_starting_distance);
        Game {
            id: gid, 
            name: name,
            year: STARTING_YEAR,
            parameters: parameters,
            universe: u
        }
    }
}
