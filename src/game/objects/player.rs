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
use ::game::objects::tech::ResearchField;
use ::game::objects::race::Race;
use ::game::objects::tech::TechnologyId;
use ::game::objects::tech::Technology;
use ::game::objects::tech::TECHNOLOGY_DETAILS;

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
    pub learned_tech_ids: Vec<TechnologyId>
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
            available_tech_ids: available_techs
        }
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
