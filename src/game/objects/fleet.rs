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
use ::game::objects::universe::SpaceCoordinate;
use ::std::cmp::Ordering;

pub const MAX_SHIP_DESIGNS : u8 = 16;

//std::fmt for strings

#[derive(Serialize, Deserialize)]
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
    Patrol
}

#[derive(Serialize, Deserialize)]
pub struct ShipOrder {
    pub order_type: ShipOrderType,
    pub amount: u16
}

#[derive(Serialize, Deserialize)]
pub struct ShipDesign {
    pub id: u32,
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
    pub owner_id: u8,
    pub location: SpaceCoordinate,
    pub heading: SpaceCoordinate,
    pub orders: Vec<ShipOrder>,
    pub repeat_orders: bool,
    pub members: Vec<FleetMember>
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
