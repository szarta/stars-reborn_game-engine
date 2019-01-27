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
use ::game::objects::tech::TechnologyId;
use ::game::objects::tech::TECHNOLOGY_DETAILS;
use ::game::objects::universe::SpaceCoordinate;
use ::game::objects::universe::Universe;
use ::std::cmp::Ordering;

pub const MAX_SHIP_DESIGNS : u8 = 16;

//std::fmt for strings

#[derive(Serialize, Deserialize, Clone)]
pub struct ShipSlot {
    pub tid: TechnologyId,
    pub amount: u8
}

#[derive(Serialize, Deserialize)]
pub enum ShipOrderType {
    NoTask,
    Load,
    Unload,
    Colonize,
    Scrap,
    Patrol,
    RemoteMining,
    LayMines,
    Route,
    Merge,
    Transfer
}

#[derive(Serialize, Deserialize)]
pub struct ShipOrder {
    pub order_type: ShipOrderType,
    pub amount: Option<u16>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShipDesign {
    pub id: u32,
    pub icon_index: u8,
    pub name: String,
    pub base_hull: TechnologyId,
    pub slots: Option<[Option<ShipSlot>; 16]>
}

#[derive(Serialize, Deserialize)]
pub struct FleetMember {
    pub design_id: u32,
    pub quantity: u16
}

#[derive(Serialize, Deserialize)]
pub struct Fleet {
    pub id: u32,
    pub owner_id: Option<u8>,
    pub location: SpaceCoordinate,
    pub heading: Option<SpaceCoordinate>,
    pub warp: Option<u8>,
    pub current_fuel: u32,
    pub total_fuel_capacity: u32,
    //pub total_cargo_capacity: Option<u32>,
    // current damage level
    // current fuel level
    // cloaking??
    pub orders: Vec<ShipOrder>,
    pub repeat_orders: bool,
    pub members: Vec<FleetMember>
}

impl Fleet {
    pub fn calculate_total_fuel_capacity(universe: &Universe, members : &Vec<FleetMember>) -> u32 {
        let mut total_fuel = 0;

        for member in members.iter() {
            let d = universe.lookup_ship_design(member.design_id);
            let quantity = member.quantity as u32;

            match d {
                Some(design) => {
                    info!("hull={}, quantity={}", design.base_hull as usize, quantity);
                    let hull = &TECHNOLOGY_DETAILS[design.base_hull as usize];
                    info!("armor={}", hull.armor.unwrap());
                    total_fuel += hull.fuel.unwrap() * quantity;
                    match design.slots {
                        Some(slots) => {
                            for s in slots.iter() {
                                match s {
                                    Some(ship_slot) => {
                                        let tech = &TECHNOLOGY_DETAILS[ship_slot.tid as usize];
                                        match tech.fuel {
                                            Some(f) => {
                                                total_fuel += f * (ship_slot.amount as u32) * quantity;
                                            },
                                            None => {}
                                        }
                                    },
                                    None => {}
                                }
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }

        return total_fuel;
    }
}

impl PartialOrd for Fleet {
    fn partial_cmp(&self, other: &Fleet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fleet {
    fn eq(&self, other: &Fleet) -> bool {
        self.id == other.id
    }
}

impl Ord for Fleet {
    fn cmp(&self, other: &Fleet) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Fleet {}
