/*
 *  Copyright 2019 Brandon Arrendondo
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
use ::game::objects::fleet::ShipDesign;
use ::game::objects::fleet::ShipSlot;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum ShipId {
    SantaMaria = 0,
    ArmedProbe = 1,
    LongRangeScout = 2,
    StalwartDefender = 3,
    Mayflower = 4,
    Pinta = 5,
    Swashbuckler = 6,
    SporeCloud = 7,
    Teamster = 8,
    CottonPicker = 9,
    SmaugarianPeepingTom = 10,
    PotatoBug = 11,
    ShadowTransport = 12,
}

pub const ORIGINAL_GAME_SHIP_NAMES : &'static [&'static str] = &[
    "Santa Maria",
    "Armed Probe",
    "Long Range Scout",
    "Stalwart Defender",
    "Mayflower",
    "Pinta",
    "Swashbuckler",
    "Spore Cloud",
    "Teamster",
    "Cotton Picker",
    "Smaugarian Peeping Tom",
    "Potato Bug",
    "Shadow Transport",
];

//pub const NUMBER_OF_ORIGINAL_SHIP_DESIGNS : usize = ORIGINAL_GAME_SHIP_NAMES.len();
pub const NUMBER_OF_ORIGINAL_SHIP_DESIGNS : usize = 13;


pub fn construct_initial_ship_designs(best_engine: TechnologyId, best_laser: TechnologyId, best_shield: TechnologyId, best_scanner: TechnologyId, best_miner: TechnologyId) -> [ShipDesign; NUMBER_OF_ORIGINAL_SHIP_DESIGNS]  {
    let santa_maria = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::SantaMaria as usize].to_string(),
        base_hull: TechnologyId::ColonyShip,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::ColonizationModule,
                amount: 1
            }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let shadow_transport = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::ShadowTransport as usize].to_string(),
        base_hull: TechnologyId::SmallFreighter,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_shield,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::TransportCloaking,
                amount: 1
            }),
            None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let potato_bug = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::PotatoBug as usize].to_string(),
        base_hull: TechnologyId::MidgetMiner,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_miner,
                amount: 2
            }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let smaugarian_peeping_tom = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::SmaugarianPeepingTom as usize].to_string(),
        base_hull: TechnologyId::Scout,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::FuelTank,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let teamster = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::Teamster as usize].to_string(),
        base_hull: TechnologyId::MediumFreighter,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::Crobmnium,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let cotton_picker = ShipDesign {
        id: 0,
        icon_index: 1,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::CottonPicker as usize].to_string(),
        base_hull: TechnologyId::MiniMiner,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_miner,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_miner,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let stalwart_defender = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::StalwartDefender as usize].to_string(),
        base_hull: TechnologyId::Destroyer,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_laser,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::AlphaTorpedo,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::Crobmnium,
                amount: 2
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::BattleComputer,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::FuelTank,
                amount: 1
            }),
            None, None, None,
            None, None, None, None, None, None
        ])
    };

    let swashbuckler = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::Swashbuckler as usize].to_string(),
        base_hull: TechnologyId::Privateer,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::Crobmnium,
                amount: 2
            }),
            Some(ShipSlot {
                tid: best_laser,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::AlphaTorpedo,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let long_range_scout = ShipDesign {
        id: 0,
        icon_index: 0,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::LongRangeScout as usize].to_string(),
        base_hull: TechnologyId::Scout,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::FuelTank,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let armed_probe = ShipDesign {
        id: 0,
        icon_index: 1,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::ArmedProbe as usize].to_string(),
        base_hull: TechnologyId::Scout,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_laser,
                amount: 1
            }),
            Some(ShipSlot {
                tid: best_scanner,
                amount: 1
            }),
            None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let pinta = ShipDesign {
        id: 0,
        icon_index: 1,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::Pinta as usize].to_string(),
        base_hull: TechnologyId::ColonyShip,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::OrbitalConstructionModule,
                amount: 1
            }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let spore_cloud = ShipDesign {
        id: 0,
        icon_index: 1,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::SporeCloud as usize].to_string(),
        base_hull: TechnologyId::MiniColonyShip,
        slots: Some([
            Some(ShipSlot {
                tid: TechnologyId::SettlersDelight,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::ColonizationModule,
                amount: 1
            }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    let mayflower = ShipDesign {
        id: 0,
        icon_index: 2,
        name: ORIGINAL_GAME_SHIP_NAMES[ShipId::Mayflower as usize].to_string(),
        base_hull: TechnologyId::ColonyShip,
        slots: Some([
            Some(ShipSlot {
                tid: best_engine,
                amount: 1
            }),
            Some(ShipSlot {
                tid: TechnologyId::ColonizationModule,
                amount: 1
            }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        ])
    };

    return [
        santa_maria, 
        armed_probe, 
        long_range_scout,
        stalwart_defender,
        mayflower,
        pinta,
        swashbuckler,
        spore_cloud,
        teamster,
        cotton_picker,
        smaugarian_peeping_tom,
        potato_bug,
        shadow_transport
    ];
}

pub const ORIGINAL_GAME_STARBASE_NAMES : &'static [&'static str] = &[
    "Space Station",
    "Portal to Nowhere"
];
