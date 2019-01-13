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
use ::game::objects::universe::SpaceCoordinate;
use ::game::objects::universe::MineralContents;
use ::game::objects::player::Player;


#[derive(Serialize, Deserialize)]
pub struct HabitatLevel {
    pub temperature: u8,
    pub gravity: u8,
    pub radiation: u8
}

#[derive(Serialize, Deserialize)]
pub enum DefenseQuality {
    SDI,
    Missile,
    Laser,
    Planet,
    Neutron
}

pub enum PlanetDesigns {
    Mines,
    Factories,
    Defenses,
    MineralAlchemy
}

pub const TEMPERATURE_DISPLAY_LEVELS : &'static [&'static str] = &[
    "-200", "-196", "-192", "-188", "-184", "-180", "-176", "-172", "-168",
    "-164", "-160", "-156", "-152", "-148", "-144", "-140", "-136", "-132",
    "-128", "-124", "-120", "-116", "-112", "-108", "-104", "-100", "-96",
    "-92", "-88", "-84", "-80", "-76", "-72", "-68", "-64", "-60", "-56", "-52",
    "-48", "-44", "-40", "-36", "-32", "-28", "-24", "-20", "-16", "-12", "-8",
    "-4", "0", "4", "8", "12", "16", "20", "24", "28", "32", "36", "40", "44",
    "48", "52", "56", "60", "64", "68", "72", "76", "80", "84", "88", "92",
    "96", "100", "104", "108", "112", "116", "120", "124", "128", "132", "136",
    "140", "144", "148", "152", "156", "160", "164", "168", "172", "176", "180",
    "184", "188", "192", "196", "200"
];

pub fn temperature_display_level_to_habitat_level(display_level : &str) -> u8 {
    let index = TEMPERATURE_DISPLAY_LEVELS.iter().position( |&r|
        r == display_level).unwrap();

    return index as u8;
}

pub const GRAVITY_DISPLAY_LEVELS : &'static [&'static str] = &[
    "0.12", "0.13", "0.14", "0.15", "0.16", "0.17", "0.18", "0.19", "0.20",
    "0.21", "0.22", "0.24", "0.25", "0.27", "0.29", "0.31", "0.33", "0.36",
    "0.40", "0.44", "0.50", "0.51", "0.52", "0.53", "0.54", "0.55", "0.56",
    "0.58", "0.59", "0.60", "0.62", "0.64", "0.65", "0.67", "0.69", "0.71",
    "0.73", "0.75", "0.78", "0.80", "0.83", "0.86", "0.89", "0.92", "0.96",
    "1.00", "1.04", "1.08", "1.12", "1.16", "1.20", "1.24", "1.28", "1.32",
    "1.36", "1.40", "1.44", "1.48", "1.52", "1.56", "1.60", "1.64", "1.68",
    "1.72", "1.76", "1.80", "1.84", "1.88", "1.92", "1.96", "2.00", "2.24",
    "2.48", "2.72", "2.96", "3.20", "3.44", "3.68", "3.92", "4.16", "4.40",
    "4.64", "4.88", "5.12", "5.36", "5.60", "5.84", "6.08", "6.32", "6.56",
    "6.80", "7.04", "7.28", "7.52", "7.76", "8.00"
];

pub fn gravity_display_level_to_habitat_level(display_level : &str) -> u8 {
    let index = GRAVITY_DISPLAY_LEVELS.iter().position( |&r|
        r == display_level).unwrap();

    return index as u8;
}

pub const RADIATION_DISPLAY_LEVELS : &'static [&'static str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
    "100"
];

pub fn radiation_display_level_to_habitat_level(display_level : &str) -> u8 {
    let index = RADIATION_DISPLAY_LEVELS.iter().position( |&r|
        r == display_level).unwrap();

    return index as u8;
}

#[derive(Serialize, Deserialize)]
pub struct BuildItem {
    pub quantity: u16,
    pub percent_complete: u8,
    pub design_id: u16,
    pub required_resources: u32,
    pub required_minerals: MineralContents,
    pub estimated_completion_year: Option<u32>
}

#[derive(Serialize, Deserialize)]
pub struct PlanetShortSummary {
    pub id: u32,
    pub name: String,
    pub x: u16,
    pub y: u16
}

impl PlanetShortSummary {
    pub fn construct_from_planet(p: &Planet) -> PlanetShortSummary {
        PlanetShortSummary {
            id: p.id,
            name: p.name.to_string(),
            x: p.location.x,
            y: p.location.y
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Planet {
    pub id: u32,
    pub location: SpaceCoordinate,
    pub name: String,

    pub mines: u16,
    pub factories: u16,
    pub defenses: u16,
    pub defense_quality: Option<DefenseQuality>,

    pub population: u32,
    pub owner_id: Option<u8>,
    pub production_queue: Vec<BuildItem>,
    pub is_homeworld: bool,
    pub related_fleets: Vec<u32>,
    pub related_starbase: Option<u32>,
    pub packet_warpspeed: Option<u8>,
    pub packet_destination_id: Option<u32>,
    pub has_ever_been_colonized: bool,
    pub mineral_concentration: MineralContents,
    pub on_surface: MineralContents,
    pub habitat: HabitatLevel
}

impl Planet {
    pub fn set_homeworld(&mut self, player: &mut Player) {
        player.homeworld_id = self.id;
        self.is_homeworld = true;

        // The ideal value for habitat stays random for an immunity
        if !player.race.gravity_immune {
            self.habitat.gravity = player.race.ideal_gravity();
        }

        if !player.race.radiation_immune {
            self.habitat.radiation = player.race.ideal_radiation();
        }

        if !player.race.temperature_immune {
            self.habitat.temperature = player.race.ideal_temperature();
        }
    }

    pub fn construct_with_defaults(name: &str, id: u32, x: u16, y: u16) -> Planet {
        Planet {
            id: id,
            location: SpaceCoordinate {
                x: x,
                y: y
            },
            name: name.to_string(),
            mines: 0,
            factories: 0,
            defenses: 0,
            defense_quality: None,
            population: 0,
            owner_id: None,
            is_homeworld: false,
            related_fleets: Vec::new(),
            production_queue: Vec::new(),
            related_starbase: None,
            packet_warpspeed: None,
            packet_destination_id: None,
            has_ever_been_colonized: false,
            mineral_concentration: MineralContents {
                ironium: 0,
                boranium: 0,
                germanium: 0
            },
            on_surface: MineralContents {
                ironium: 0,
                boranium: 0,
                germanium: 0
            },
            habitat: HabitatLevel {
                gravity: 0,
                temperature: 0,
                radiation: 0
            }
        }
    }
}

