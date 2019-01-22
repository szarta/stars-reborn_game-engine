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
use ::game::objects::race::Race;
use ::game::objects::race::PrimaryRacialTrait;
use ::game::objects::tech::ResearchField;
use ::game::objects::tech::TechnologyId;
use ::game::objects::tech::Technology;
use ::game::objects::tech::TECHNOLOGY_DETAILS;
use ::game::objects::universe::Universe;
use ::game::objects::fleet::ShipDesign;
use ::game::objects::fleet::ShipSlot;
use ::game::objects::fleet::MAX_SHIP_DESIGNS;

use ::game::objects::predefined::fleets::ShipId;
use ::game::objects::predefined::fleets::construct_initial_ship_designs;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: u8,
    pub race: Race,
    pub homeworld_id: u32,

    pub research_budget: u8,
    pub tech_level: [u8; 6],
    pub tech_progress: [u32; 6],
    pub current_research_field: ResearchField,
    pub next_research_field: ResearchField,
    pub available_tech_ids: Vec<TechnologyId>,
    pub learned_tech_ids: Vec<TechnologyId>,
    pub ship_designs: [Option<ShipDesign>; MAX_SHIP_DESIGNS as usize]
}

impl Player {
    pub fn construct_from_race(race: Race) -> Player {
        let tech_level: [u8; 6] = race.calculate_starting_tech_levels();
        let tech_progress: [u32; 6] = [0,0,0,0,0,0];
        let available_techs: Vec<TechnologyId> = race.calculate_initial_available_technologies();
        let learned_techs: Vec<TechnologyId> = calculate_initial_learned_technologies(&available_techs, tech_level);

        Player {
            id: 0,
            race: race,
            homeworld_id: 0,

            research_budget: 15,
            tech_level: tech_level,
            tech_progress: tech_progress,
            current_research_field: ResearchField::Energy,
            next_research_field: ResearchField::Energy,

            learned_tech_ids: learned_techs,
            available_tech_ids: available_techs,
            ship_designs: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None ]

        }
    }

    pub fn get_next_available_ship_design_slot(&self) -> Option<u32> {
        for i in 0..MAX_SHIP_DESIGNS {
            if !Option::is_some(&self.ship_designs[i as usize]) {
                return Some(i as u32);
            }
        }

        return None;
    }

    pub fn get_ship_design_id(&self) -> Option<u32> {
        let i = self.get_next_available_ship_design_slot();
        match i {
            Some(index) => {
                return Some((self.id as u32 * MAX_SHIP_DESIGNS as u32) + index);
            }
            None => {
                return None;
            }
        }
    }

    pub fn add_ship_design(&mut self, mut d: ShipDesign) -> bool {
        match self.get_next_available_ship_design_slot() {
            Some(index) => {
                let id = self.get_ship_design_id().unwrap();
                d.id = id;
                self.ship_designs[id as usize] = Some(d);
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub fn get_best_starting_engine(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::FuelMizer) {
            return TechnologyId::FuelMizer.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::LongHump6) {
            return TechnologyId::LongHump6.clone();
        }

        return TechnologyId::QuickJump5.clone();
    }

    pub fn generate_initial_ship_designs(&mut self) {
        let best_engine = self.get_best_starting_engine();
        let ship_designs = construct_initial_ship_designs(best_engine);

        match self.race.primary_racial_trait {
            PrimaryRacialTrait::ClaimAdjuster => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
            },
            PrimaryRacialTrait::JackOfAllTrades => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                self.add_ship_design(ship_designs[ShipId::LongRangeScout as usize].clone());
                self.add_ship_design(ship_designs[ShipId::StalwartDefender as usize].clone());
                self.add_ship_design(ship_designs[ShipId::Swashbuckler as usize].clone());
                self.add_ship_design(ship_designs[ShipId::ArmedProbe as usize].clone());
            },
            PrimaryRacialTrait::InterstellarTraveler => {
                self.add_ship_design(ship_designs[ShipId::Mayflower as usize].clone());
            },
            PrimaryRacialTrait::InnerStrength => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
            },
            PrimaryRacialTrait::SpaceDemolition => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
            },
            PrimaryRacialTrait::WarMonger => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
            },
            PrimaryRacialTrait::PacketPhysics => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
                self.add_ship_design(ship_designs[ShipId::LongRangeScout as usize].clone());
            },
            PrimaryRacialTrait::SuperStealth => {
                self.add_ship_design(ship_designs[ShipId::SantaMaria as usize].clone());
            },
            PrimaryRacialTrait::HyperExpansion => {
                self.add_ship_design(ship_designs[ShipId::SporeCloud as usize].clone());
            },
            PrimaryRacialTrait::AlternateReality => {
                self.add_ship_design(ship_designs[ShipId::Pinta as usize].clone());
            }
        }
    }

    pub fn generate_initial_ships(&self, universe: &Universe) {

        /*
        match self.race.primary_racial_trait {
            PrimaryRacialTrait::ClaimAdjuster => {},
            PrimaryRacialTrait::JackOfAllTrades => {



            },
            PrimaryRacialTrait::InterstellarTraveler => {},
            PrimaryRacialTrait::InnerStrength => {},
            PrimaryRacialTrait::SpaceDemolition => {},
            PrimaryRacialTrait::WarMonger => {},
            PrimaryRacialTrait::PacketPhysics => {},
            PrimaryRacialTrait::SuperStealth => {},
            PrimaryRacialTrait::HyperExpansion => {},
            PrimaryRacialTrait::AlternateReality => {}
        }
        */
    }
}

pub fn calculate_initial_learned_technologies(available_techs: &Vec<TechnologyId>, tech_level: [u8; 6]) -> Vec<TechnologyId> {
    let mut ret : Vec<TechnologyId> = Vec::new();

    for tid in available_techs.iter() {
        let t : &Technology = &TECHNOLOGY_DETAILS[*tid as usize];
        if tech_level[0] >= t.requirement.levels[0] &&
           tech_level[1] >= t.requirement.levels[1] &&
           tech_level[2] >= t.requirement.levels[2] &&
           tech_level[3] >= t.requirement.levels[3] &&
           tech_level[4] >= t.requirement.levels[4] &&
           tech_level[5] >= t.requirement.levels[5] {

            ret.push(tid.clone());
        }
    }

    return ret;
}
