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
use uuid::Uuid;

use ::game::objects::universe::Universe;
use ::game::objects::universe::UniverseSize;
use ::game::objects::universe::UniverseDensity;
use ::game::objects::universe::SpaceCoordinate;
use ::game::objects::game::Game;
use ::game::objects::game::GameParameters;
use ::game::objects::game::VictoryConditions;
use ::game::objects::game::PlayerStartingDistance;
use ::game::objects::planet::Planet;
use ::game::objects::player::Player;
use ::game::objects::planet::temperature_display_level_to_habitat_level;
use ::game::objects::planet::radiation_display_level_to_habitat_level;
use ::game::objects::planet::gravity_display_level_to_habitat_level;
use ::game::objects::predefined::races::PredefinedRace;
use ::game::objects::predefined::races::PredefinedCPURace;
use ::game::objects::predefined::races::CPUDifficulty;

use std::collections::HashMap;

pub fn generate_tutorial_game() -> Game {
    let gid = Uuid::new_v4().to_string();
    let mut u = generate_tutorial_universe();
    let mut players = Vec::new();

    let p1_race = PredefinedRace::Humanoid.generate();
    let mut p1 = Player::construct_from_race(p1_race);
    p1.id = 0;
    p1.generate_initial_ship_designs();

    u.planets[13].set_homeworld(&mut p1);
    u.planets[13].population = p1.race.calculate_starting_population(UniverseSize::Tiny);
    players.push(p1);

    let mut p2_race = PredefinedCPURace::Robotoids.generate(CPUDifficulty::Expert);
    p2_race.name = "Berserker".to_string();
    p2_race.plural_name = "Berserkers".to_string();
    let mut p2 = Player::construct_from_race(p2_race);
    p2.id = 1;

    u.planets[10].set_homeworld(&mut p2);
    u.planets[10].population = p2.race.calculate_starting_population(UniverseSize::Tiny);
    players.push(p2);

    let vc = VictoryConditions {
        owns_percent_planets: false,
        percent_planets: 0,
        obtains_tech_levels: false,
        required_tech_level: 0,
        number_tech_fields: 0,
        exceeds_score: false,
        score_to_exceed: 0,
        exceeds_second_place_score: false,
        exceeds_second_place_by: 0,
        has_production_capacity: false,
        minimum_production_capacity: 0,
        owns_capital_ships: false,
        minimum_capital_ships: 0,
        has_highest_score: true,
        highest_score_years: 30,
        number_of_criteria: 1,
        minimum_years: 30
    };

    let params = GameParameters {
        allow_random_events: false,
        accelerated_play: true,
        public_player_scores: true,
        maximum_minerals: false,
        slow_tech_advances: false,
        galaxy_clumping: false,
        player_starting_distance: PlayerStartingDistance::Moderate,
        universe_size: UniverseSize::Tiny,
        universe_density: UniverseDensity::Sparse,
        victory_conditions: vc
    };

    Game {
        id: gid,
        name: "Tutorial Game".to_string(),
        year: ::game::objects::game::STARTING_YEAR,
        players: players,
        parameters: params,
        universe: u
    }
}

pub fn generate_tutorial_universe() -> Universe {
    let mut boundaries = Vec::new();
    boundaries.push(SpaceCoordinate { x: 0, y: 0 });
    boundaries.push(SpaceCoordinate { x: 0, y: 400 });
    boundaries.push(SpaceCoordinate { x: 400, y: 400 });
    boundaries.push(SpaceCoordinate { x: 400, y: 0 });

    let mut tut = Universe {
        boundary: boundaries,
        minefields: Vec::new(),
        mineral_packets: Vec::new(),
        salvage: Vec::new(),
        wormholes: Vec::new(),
        planets: Vec::new(),
        fleets: HashMap::new()
    };

    let mut p = Planet::construct_with_defaults("Lever", 1, 26, 345);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.67");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-44");
    p.habitat.radiation = radiation_display_level_to_habitat_level("46");
    p.mineral_concentration.ironium = 24;
    p.mineral_concentration.boranium = 70;
    p.mineral_concentration.germanium = 84;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Speed Bump", 2, 74, 369);
    p.habitat.gravity = gravity_display_level_to_habitat_level("2.00");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-116");
    p.habitat.radiation = radiation_display_level_to_habitat_level("59");
    p.mineral_concentration.ironium = 57;
    p.mineral_concentration.boranium = 72;
    p.mineral_concentration.germanium = 64;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Oxygen", 3, 85, 187);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.60");
    p.habitat.temperature = temperature_display_level_to_habitat_level("104");
    p.habitat.radiation = radiation_display_level_to_habitat_level("13");
    p.mineral_concentration.ironium = 89;
    p.mineral_concentration.boranium = 92;
    p.mineral_concentration.germanium = 88;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("No Vacancy", 4, 105, 129);
    p.habitat.gravity = gravity_display_level_to_habitat_level("3.20");
    p.habitat.temperature = temperature_display_level_to_habitat_level("28");
    p.habitat.radiation = radiation_display_level_to_habitat_level("98");
    p.mineral_concentration.ironium = 85;
    p.mineral_concentration.boranium = 46;
    p.mineral_concentration.germanium = 43;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Mozart", 5, 118, 257);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.44");
    p.habitat.temperature = temperature_display_level_to_habitat_level("112");
    p.habitat.radiation = radiation_display_level_to_habitat_level("27");
    p.mineral_concentration.ironium = 83;
    p.mineral_concentration.boranium = 22;
    p.mineral_concentration.germanium = 13;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Wallaby", 6, 121, 203);
    p.habitat.gravity = gravity_display_level_to_habitat_level("3.44");
    p.habitat.temperature = temperature_display_level_to_habitat_level("16");
    p.habitat.radiation = radiation_display_level_to_habitat_level("92");
    p.mineral_concentration.ironium = 80;
    p.mineral_concentration.boranium = 100;
    p.mineral_concentration.germanium = 61;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("La Te Da", 7, 127, 369);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.21");
    p.habitat.temperature = temperature_display_level_to_habitat_level("48");
    p.habitat.radiation = radiation_display_level_to_habitat_level("32");
    p.mineral_concentration.ironium = 20;
    p.mineral_concentration.boranium = 70;
    p.mineral_concentration.germanium = 2;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Mohlodi", 8, 130, 28);
    p.habitat.gravity = gravity_display_level_to_habitat_level("5.84");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-36");
    p.habitat.radiation = radiation_display_level_to_habitat_level("44");
    p.mineral_concentration.ironium = 25;
    p.mineral_concentration.boranium = 26;
    p.mineral_concentration.germanium = 18;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Slime", 9, 131, 184);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.51");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-60");
    p.habitat.radiation = radiation_display_level_to_habitat_level("13");
    p.mineral_concentration.ironium = 4;
    p.mineral_concentration.boranium = 2;
    p.mineral_concentration.germanium = 6;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Hiho", 10, 148, 104);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.68");
    p.habitat.temperature = temperature_display_level_to_habitat_level("72");
    p.habitat.radiation = radiation_display_level_to_habitat_level("9");
    p.mineral_concentration.ironium = 68;
    p.mineral_concentration.boranium = 77;
    p.mineral_concentration.germanium = 23;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Hacker", 11, 149, 265);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.32");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-60");
    p.habitat.radiation = radiation_display_level_to_habitat_level("65");
    p.mineral_concentration.ironium = 25;
    p.mineral_concentration.boranium = 64;
    p.mineral_concentration.germanium = 75;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Neil", 12, 157, 255);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.56");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-160");
    p.habitat.radiation = radiation_display_level_to_habitat_level("82");
    p.mineral_concentration.ironium = 4;
    p.mineral_concentration.boranium = 84;
    p.mineral_concentration.germanium = 103;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Prune", 13, 190, 112);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.08");
    p.habitat.temperature = temperature_display_level_to_habitat_level("160");
    p.habitat.radiation = radiation_display_level_to_habitat_level("62");
    p.mineral_concentration.ironium = 87;
    p.mineral_concentration.boranium = 41;
    p.mineral_concentration.germanium = 109;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Stove Top", 14, 237, 127);
    p.mineral_concentration.ironium = 70;
    p.mineral_concentration.boranium = 84;
    p.mineral_concentration.germanium = 25;
    p.on_surface.ironium = 424;
    p.on_surface.boranium = 477;
    p.on_surface.germanium = 622;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Shaggy Dog", 15, 263, 270);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.56");
    p.habitat.temperature = temperature_display_level_to_habitat_level("20");
    p.habitat.radiation = radiation_display_level_to_habitat_level("74");
    p.mineral_concentration.ironium = 82;
    p.mineral_concentration.boranium = 44;
    p.mineral_concentration.germanium = 91;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Alexander", 16, 263, 186);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.13");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-76");
    p.habitat.radiation = radiation_display_level_to_habitat_level("7");
    p.mineral_concentration.ironium = 95;
    p.mineral_concentration.boranium = 11;
    p.mineral_concentration.germanium = 8;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("90210", 17, 295, 131);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.76");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-120");
    p.habitat.radiation = radiation_display_level_to_habitat_level("44");
    p.mineral_concentration.ironium = 85;
    p.mineral_concentration.boranium = 82;
    p.mineral_concentration.germanium = 62;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Sea Squared", 18, 320, 294);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.33");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-124");
    p.habitat.radiation = radiation_display_level_to_habitat_level("19");
    p.mineral_concentration.ironium = 107;
    p.mineral_concentration.boranium = 96;
    p.mineral_concentration.germanium = 47;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Red Storm", 19, 352, 254);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.56");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-84");
    p.habitat.radiation = radiation_display_level_to_habitat_level("48");
    p.mineral_concentration.ironium = 96;
    p.mineral_concentration.boranium = 71;
    p.mineral_concentration.germanium = 76;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Mobius", 20, 366, 53);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.17");
    p.habitat.temperature = temperature_display_level_to_habitat_level("68");
    p.habitat.radiation = radiation_display_level_to_habitat_level("86");
    p.mineral_concentration.ironium = 85;
    p.mineral_concentration.boranium = 72;
    p.mineral_concentration.germanium = 97;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Castle", 21, 366, 15);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.13");
    p.habitat.temperature = temperature_display_level_to_habitat_level("172");
    p.habitat.radiation = radiation_display_level_to_habitat_level("39");
    p.mineral_concentration.ironium = 70;
    p.mineral_concentration.boranium = 92;
    p.mineral_concentration.germanium = 21;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Dwarte", 22, 382, 98);
    p.habitat.gravity = gravity_display_level_to_habitat_level("3.68");
    p.habitat.temperature = temperature_display_level_to_habitat_level("104");
    p.habitat.radiation = radiation_display_level_to_habitat_level("98");
    p.mineral_concentration.ironium = 98;
    p.mineral_concentration.boranium = 61;
    p.mineral_concentration.germanium = 53;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Kalamazoo", 23, 385, 196);
    p.habitat.gravity = gravity_display_level_to_habitat_level("0.24");
    p.habitat.temperature = temperature_display_level_to_habitat_level("-96");
    p.habitat.radiation = radiation_display_level_to_habitat_level("79");
    p.mineral_concentration.ironium = 4;
    p.mineral_concentration.boranium = 93;
    p.mineral_concentration.germanium = 50;
    tut.planets.push(p);

    p = Planet::construct_with_defaults("Bloop", 24, 386, 218);
    p.habitat.gravity = gravity_display_level_to_habitat_level("1.44");
    p.habitat.temperature = temperature_display_level_to_habitat_level("136");
    p.habitat.radiation = radiation_display_level_to_habitat_level("34");
    p.mineral_concentration.ironium = 102;
    p.mineral_concentration.boranium = 20;
    p.mineral_concentration.germanium = 62;
    tut.planets.push(p);

    return tut;
}
