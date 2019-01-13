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
use ::game::objects::game::PlayerStartingDistance;
use ::game::objects::player::Player;
use ::game::objects::planet::Planet;
use rand;
use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct SpaceCoordinate {
    pub x: u16,
    pub y: u16
}

#[derive(Serialize, Deserialize)]
pub struct Heading {
    pub warp: u8,
    pub next_location: SpaceCoordinate
}

#[derive(Serialize, Deserialize)]
pub struct MineralContents {
    pub boranium: u16,
    pub germanium: u16,
    pub ironium: u16
}

#[derive(Serialize, Deserialize)]
pub enum WormholeStability {
    Stable,
    RockSolid,
    Volatile
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum UniverseDensity {
    Sparse,
    Normal,
    Dense,
    Packed
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum UniverseSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge
}

impl UniverseSize {
    fn value(&self) -> u16 {
        match *self {
            UniverseSize::Tiny => 400,
            UniverseSize::Small => 800,
            UniverseSize::Medium => 1200,
            UniverseSize::Large => 1600,
            UniverseSize::Huge => 2000
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Wormhole {
    pub id: u32,
    pub location: SpaceCoordinate,

    pub stability: WormholeStability,
    pub other_end_id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Salvage {
    id: u32,
    location: SpaceCoordinate,

    minerals: MineralContents
}

#[derive(Serialize, Deserialize)]
pub struct Minefield {
    id: u32,
    location: SpaceCoordinate,

    owner_id: u8
}

#[derive(Serialize, Deserialize)]
pub struct MineralPacket {
    id: u32,
    location: SpaceCoordinate,

    heading: Heading,
    owner_id: u8,
    minerals: MineralContents
}

pub fn get_coordinate_square(dimension: u16) -> Vec<SpaceCoordinate> {
    let mut boundaries = Vec::new();
    boundaries.push(SpaceCoordinate { x: 0, y: 0 });
    boundaries.push(SpaceCoordinate { x: 0, y: dimension });
    boundaries.push(SpaceCoordinate { x: dimension, y: dimension });
    boundaries.push(SpaceCoordinate { x: dimension, y: 0 });

    return boundaries;
}

pub fn get_density_value(size: &UniverseSize, density: &UniverseDensity) -> f64 {
    match size {
        UniverseSize::Tiny => match density {
            UniverseDensity::Sparse => 1.0,
            UniverseDensity::Normal => 1.0,
            UniverseDensity::Dense => 1.0,
            UniverseDensity::Packed => 1.0
        },
        UniverseSize::Small => match density {
            UniverseDensity::Sparse => 1.0,
            UniverseDensity::Normal => 1.0,
            UniverseDensity::Dense => 1.0,
            UniverseDensity::Packed => 1.0
        },
        UniverseSize::Medium => match density {
            UniverseDensity::Sparse => 1.0,
            UniverseDensity::Normal => 1.0,
            UniverseDensity::Dense => 1.0,
            UniverseDensity::Packed => 1.0
        },
        UniverseSize::Large => match density {
            UniverseDensity::Sparse => 1.0,
            UniverseDensity::Normal => 1.0,
            UniverseDensity::Dense => 1.0, 
            UniverseDensity::Packed => 1.0
        },
        UniverseSize::Huge => match density {
            UniverseDensity::Sparse => 1.0,
            UniverseDensity::Normal => 1.0,
            UniverseDensity::Dense => 1.0,
            UniverseDensity::Packed => 2.38 as f64,
        }
    }
}

pub fn generate_random_planet_configuration(size: &UniverseSize, density: &UniverseDensity, galaxy_clumping: &bool) -> Vec<Planet> {
    let ret = Vec::new();

    // Universe generation is crucial to replicating original gameplay and is 
    // not fully detailed via wiki/forums.
    //
    // One piece of universe generation is planet placement.
    //
    // The goal is to place planets in such a way that the generated map is
    // indistinguishable from a map generated via the original Stars! game.
    //
    // In the stars-research project, I generated 1000 maps of each variant of
    // universe size, density, and with/without galaxy clumping.  Then I ran 
    // tools to analyze the maps to determine important criteria for the
    // placement algorithm.
    //
    // Some of the questions were:
    //
    //  * Is there a minimum distance between 2 planets?
    //  * Is there any x,y value that is always/never hit?
    //  * What are the min/max planets in a 100x100 region?
    //  * Is there any clear path for the order in which planets are placed?
    //  * Are there any obvious patterns/rules?
    //     * I.E. if 100x100 squares are buckets, are they filled the same?
    //
    // Some observations:
    //
    // * Universes start at x=1000 and y=1000
    // * There is a buffer for planets of 10 around the boundary, resulting in:
    //      * The min value for x and y is 1010, max value is (boundary - 10)
    // * In planet ID order, the x coordinate increments
    // * Density is the number of planets in a 100x100 square
    //   * Documented in density table, this is a target for the generated 
    //     universe
    // * The minimum Manhattan distance between planets is 13.
    //   * In this case, the minimum distance on one of the axes is 12.
    // * As a last step, the planets are pulled closer for galaxy clumping

    let mut rng = rand::thread_rng();

    let multiplier: f64 = rng.gen();
    let first_x = 1010 + match size {
        UniverseSize::Tiny => match density {
            UniverseDensity::Sparse => (75.0 * multiplier).round() as u16,
            UniverseDensity::Normal => (66.0 * multiplier).round() as u16,
            UniverseDensity::Dense => 5,
            UniverseDensity::Packed => (11.0 * multiplier).round() as u16,
        },
        UniverseSize::Small => match density {
            UniverseDensity::Sparse => 5,
            UniverseDensity::Normal => 5,
            UniverseDensity::Dense => 5,
            UniverseDensity::Packed => (11.0 * multiplier).round() as u16,
        },
        UniverseSize::Medium => match density {
            UniverseDensity::Sparse => 5,
            UniverseDensity::Normal => 5,
            UniverseDensity::Dense => 5,
            UniverseDensity::Packed => (11.0 * multiplier).round() as u16,
        },
        UniverseSize::Large => match density {
            UniverseDensity::Sparse => 5,
            UniverseDensity::Normal => 5,
            UniverseDensity::Dense => 5,
            UniverseDensity::Packed => (11.0 * multiplier).round() as u16,
        },
        UniverseSize::Huge => match density {
            UniverseDensity::Sparse => 5,
            UniverseDensity::Normal => 5,
            UniverseDensity::Dense => 5,
            UniverseDensity::Packed => (11.0 * multiplier).round() as u16,
        }
    };

    info!("first x: {}", first_x);


    return ret;
}

#[derive(Serialize, Deserialize)]
pub struct Universe {
    pub boundary: Vec<SpaceCoordinate>,
    pub wormholes: Vec<Wormhole>,
    pub salvage: Vec<Salvage>,
    pub minefields: Vec<Minefield>,
    pub mineral_packets: Vec<MineralPacket>,
    pub planets: Vec<::game::objects::planet::Planet>
}

impl Universe {
    pub fn construct_random(size: &UniverseSize, density: &UniverseDensity, galaxy_clumping: &bool, players: &Vec<Player>, starting_distance: &PlayerStartingDistance) -> Universe {
        let u = Universe {
            boundary: get_coordinate_square(size.value()),
            wormholes: Vec::new(),
            salvage: Vec::new(),
            minefields: Vec::new(),
            mineral_packets: Vec::new(),
            planets: generate_random_planet_configuration(size, density, galaxy_clumping)
        };

        return u;
    }
}
