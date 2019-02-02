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
use ::game::objects::tech::ResearchField;
use ::game::objects::tech::TechnologyId;
use ::game::objects::tech::Technology;
use ::game::objects::tech::TECHNOLOGY_DETAILS;
use ::game::objects::fleet::ShipDesign;
use ::game::objects::fleet::MAX_SHIP_DESIGNS;


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

    fn get_next_available_ship_design_slot(&self) -> Option<u8> {
        for i in 0..MAX_SHIP_DESIGNS {
            if !Option::is_some(&self.ship_designs[i as usize]) {
                return Some(i);
            }
        }

        return None;
    }

    fn get_next_available_ship_design_id(&self) -> Option<u32> {
        let i = self.get_next_available_ship_design_slot();
        match i {
            Some(index) => {
                return Some((self.id as u32 * MAX_SHIP_DESIGNS as u32) + index as u32);
            }
            None => {
                return None;
            }
        }
    }

    pub fn add_ship_design(&mut self, mut d: ShipDesign) -> u8 {
        match self.get_next_available_ship_design_slot() {
            Some(index) => {
                let id = self.get_next_available_ship_design_id().unwrap();
                d.id = id;
                self.ship_designs[index as usize] = Some(d);
                return index;
            }
            None => {
                error!("Tried to add ship design but no slots remain.");
                return MAX_SHIP_DESIGNS + 1;
            }
        }
    }

    pub fn get_best_starting_scanner(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::PossumScanner) {
            return TechnologyId::PossumScanner.clone();
        }

        return TechnologyId::BatScanner.clone();
    }

    pub fn get_best_starting_shield(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::CowhideShield) {
            return TechnologyId::CowhideShield.clone();
        }

        return TechnologyId::MoleskinShield.clone();
    }

    pub fn get_best_starting_laser(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::YakimoraLightPhaser) {
            return TechnologyId::YakimoraLightPhaser.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::XrayLaser) {
            return TechnologyId::XrayLaser.clone();
        }

        return TechnologyId::Laser;
    }

    pub fn get_best_starting_miner(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::RoboMiner) {
            return TechnologyId::RoboMidgetMiner.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::RoboMidgetMiner) {
            return TechnologyId::RoboMidgetMiner.clone();
        }

        return TechnologyId::RoboMiniMiner;
    }

    pub fn get_best_starting_engine(&self) -> TechnologyId {
        if self.available_tech_ids.contains(&TechnologyId::AlphaDrive8) {
            return TechnologyId::AlphaDrive8.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::DaddyLongLegs7) {
            return TechnologyId::DaddyLongLegs7.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::FuelMizer) {
            return TechnologyId::FuelMizer.clone();
        }

        if self.available_tech_ids.contains(&TechnologyId::LongHump6) {
            return TechnologyId::LongHump6.clone();
        }

        return TechnologyId::QuickJump5.clone();
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
