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
use ::game::objects::planet::Planet;
use ::game::objects::fleet::ShipDesign;
use ::game::objects::fleet::ShipOrder;
use ::game::objects::fleet::ShipOrderType;
use ::game::objects::fleet::Fleet;
use ::game::objects::fleet::FleetMember;
use ::game::objects::fleet::MAX_SHIP_DESIGNS;
use rand;
use rand::Rng;
use std::collections::HashMap;
use ::game::objects::predefined::fleets::construct_initial_ship_designs;
use ::game::objects::predefined::fleets::ShipId;
use ::game::objects::race::PrimaryRacialTrait;
use ::game::objects::race::LesserRacialTrait;

#[derive(Serialize, Deserialize, Clone)]
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
    pub planets: Vec<::game::objects::planet::Planet>,
    pub fleets: HashMap<u32, ::game::objects::fleet::Fleet>,
    pub players: Vec<::game::objects::player::Player>
}

pub const MAX_FLEETS : u32 = 30000;

impl Universe {
    pub fn construct_random(size: &UniverseSize, density: &UniverseDensity, galaxy_clumping: &bool, starting_distance: &PlayerStartingDistance) -> Universe {
        let u = Universe {
            boundary: get_coordinate_square(size.value()),
            wormholes: Vec::new(),
            salvage: Vec::new(),
            minefields: Vec::new(),
            mineral_packets: Vec::new(),
            planets: generate_random_planet_configuration(size, density, galaxy_clumping),
            fleets: HashMap::new(),
            players: Vec::new()
        };

        return u;
    }

    pub fn get_new_fleet_id(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let mut id : u32 = 0;

        while self.fleets.contains_key(&id) {
            let multiplier: f64 = rng.gen();
            id = ((MAX_FLEETS as f64) * multiplier).round() as u32;
        }

        return id;
    }

    pub fn lookup_ship_design(&self, id : u32) -> Option<ShipDesign> {
        let pid = id / (MAX_SHIP_DESIGNS as u32);
        let offset = id % (MAX_SHIP_DESIGNS as u32);
        info!("id:{}, pid:{}, offset:{}", id, pid, offset);

        let p = &self.players[pid as usize];
        let design = &p.ship_designs[offset as usize];

        match design {
            Some(d) => { Some(d.clone()) }
            None => { None }
        }
    }

    pub fn add_fleet_at_planet(&mut self, design: &ShipDesign, owner: Option<u8>, planet_id: u32, quantity: u16) -> u32 {
        let location = self.planets[planet_id as usize].location.clone();
        let fleet_id = self.add_fleet(design, owner, location, quantity);

        self.planets[planet_id as usize].related_fleets.push(fleet_id);
        return fleet_id;
    }

    pub fn add_fleet(&mut self, design: &ShipDesign, owner: Option<u8>, location: SpaceCoordinate, quantity: u16) -> u32 {
        let id : u32 = self.get_new_fleet_id();
        let mut initial_orders = Vec::new();
        initial_orders.push(ShipOrder {
            order_type: ShipOrderType::NoTask,
            amount: None
        });

        let mut members = Vec::new();
        members.push(FleetMember {
            design_id: design.id,
            quantity: quantity
        });

        let total_fuel = Fleet::calculate_total_fuel_capacity(self, &members);

        let f = Fleet {
            id: id,
            owner_id: owner,
            location: location,
            heading: None,
            warp: None,
            current_fuel: total_fuel.clone(),
            total_fuel_capacity: total_fuel.clone(),
            repeat_orders: false,
            orders: initial_orders,
            members: members
        };

        let key = f.id.clone();
        let ret = f.id.clone();
        self.fleets.insert(key, f);
        return ret;
    }

    pub fn generate_initial_ships(&mut self) {
        let mut ship_queue = Vec::new();

        for p in self.players.iter_mut() {
            let best_engine = p.get_best_starting_engine();
            let best_laser = p.get_best_starting_laser();
            let best_shield = p.get_best_starting_shield();
            let best_scanner = p.get_best_starting_scanner();
            let best_miner = p.get_best_starting_miner();
            let ship_designs = construct_initial_ship_designs(best_engine, best_laser, best_shield, best_scanner, best_miner);

            match p.race.primary_racial_trait {
                PrimaryRacialTrait::ClaimAdjuster => {
                    //p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                    //ship_queue.push((p.ship_designs[0].clone().unwrap(), p.id, p.homeworld_id, 1));
                },
                PrimaryRacialTrait::JackOfAllTrades => {
                    p.add_ship_design(ship_designs[ShipId::ArmedProbe as usize].clone());
                    p.add_ship_design(ship_designs[ShipId::LongRangeScout as usize].clone());
                    p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                    p.add_ship_design(ship_designs[ShipId::Teamster as usize].clone());
                    p.add_ship_design(ship_designs[ShipId::StalwartDefender as usize].clone());
                    p.add_ship_design(ship_designs[ShipId::CottonPicker as usize].clone());

                    ship_queue.push((p.ship_designs[0].clone().unwrap(), p.id, p.homeworld_id, 1));
                    ship_queue.push((p.ship_designs[1].clone().unwrap(), p.id, p.homeworld_id, 1));
                    ship_queue.push((p.ship_designs[2].clone().unwrap(), p.id, p.homeworld_id, 1));
                    ship_queue.push((p.ship_designs[3].clone().unwrap(), p.id, p.homeworld_id, 1));
                    ship_queue.push((p.ship_designs[4].clone().unwrap(), p.id, p.homeworld_id, 1));
                    ship_queue.push((p.ship_designs[5].clone().unwrap(), p.id, p.homeworld_id, 1));
                }
    
                PrimaryRacialTrait::InterstellarTraveler => {
                    /*
                    *p.add_ship_design(ship_designs[ShipId::SmaugarianPeepingTom as usize].clone());
                    *p.add_ship_design(ship_designs[ShipId::Mayflower as usize].clone());
                    *p.add_ship_design(ship_designs[ShipId::StalwartDefender as usize].clone());
                    *p.add_ship_design(ship_designs[ShipId::Swashbuckler as usize].clone());
                    */
                },
                PrimaryRacialTrait::InnerStrength => {
                    //*p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                },
                PrimaryRacialTrait::SpaceDemolition => {
                    //*p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                },
                PrimaryRacialTrait::WarMonger => {
                    //*p.add_ship_design(ship_designs[ShipId::ArmedProbe as usize].clone());
                    //*p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                },
                PrimaryRacialTrait::PacketPhysics => {
                    //*p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                    //*p.add_ship_design(ship_designs[ShipId::LongRangeScout as usize].clone());
                },
                PrimaryRacialTrait::SuperStealth => {
                    //*p.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                    //*p.add_ship_design(ship_designs[ShipId::ShadowTransport as usize].clone());
                },
                PrimaryRacialTrait::HyperExpansion => {
                    //*p.add_ship_design(ship_designs[ShipId::SporeCloud as usize].clone());
                },
                PrimaryRacialTrait::AlternateReality => {
                    //(&p).add_ship_design(ship_designs[ShipId::Pinta as usize].clone());
                }
            }

            if p.race.lesser_racial_traits.contains(&LesserRacialTrait::AdvancedRemoteMining) {
                let index = p.add_ship_design(ship_designs[ShipId::PotatoBug as usize].clone());
                ship_queue.push((p.ship_designs[index as usize].clone().unwrap(), p.id, p.homeworld_id, 1));
            }
        }

        for q in ship_queue {
            let (design, owner_id, homeworld_id, quantity) = q;
            self.add_fleet_at_planet(&design, Some(owner_id), homeworld_id, quantity);
        }
    }
}
