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
use std::cmp::max;
use ::game::objects::tech::ResearchField;
use game::objects::tech::TechnologyId;
use game::objects::tech::INITIAL_TECHNOLOGY;
use game::objects::tech::ADVANCED_SHIP_SCANNER_TECHNOLOGY;
use game::objects::tech::ADVANCED_PLANETARY_SCANNER_TECHNOLOGY;
use game::objects::tech::RAM_SCOOP_ENGINE_TECHNOLOGY;
use game::objects::tech::NORMAL_REMOTE_MINING_TECHNOLOGY;
use game::objects::tech::CLAIM_ADJUSTER_TECHNOLOGY;
use game::objects::tech::JACK_OF_ALL_TRADES_TECHNOLOGY;
use game::objects::tech::INTERSTELLAR_TRAVELER_TECHNOLOGY;
use game::objects::tech::INNER_STRENGTH_TECHNOLOGY;
use game::objects::tech::SPACE_DEMOLITION_TECHNOLOGY;
use game::objects::tech::WAR_MONGER_TECHNOLOGY;
use game::objects::tech::PACKET_PHYSICS_TECHNOLOGY;
use game::objects::tech::SUPER_STEALTH_TECHNOLOGY;
use game::objects::tech::HYPER_EXPANSION_TECHNOLOGY;
use game::objects::tech::ALTERNATE_REALITY_TECHNOLOGY;
use game::objects::planet::Planet;
use game::objects::universe::Universe;
use game::objects::universe::UniverseSize;

pub const BASE_STARTING_PLANET_POPULATION : u32 = 25000;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum PrimaryRacialTrait {
    ClaimAdjuster,
    JackOfAllTrades,
    InterstellarTraveler,
    InnerStrength,
    SpaceDemolition,
    WarMonger,
    PacketPhysics,
    SuperStealth,
    HyperExpansion,
    AlternateReality
}

impl PrimaryRacialTrait {
    fn value(&self) -> usize {
        match *self {
            PrimaryRacialTrait::ClaimAdjuster => 0,
            PrimaryRacialTrait::JackOfAllTrades => 1,
            PrimaryRacialTrait::InterstellarTraveler => 2,
            PrimaryRacialTrait::InnerStrength => 3,
            PrimaryRacialTrait::SpaceDemolition => 4,
            PrimaryRacialTrait::WarMonger => 5,
            PrimaryRacialTrait::PacketPhysics => 6,
            PrimaryRacialTrait::SuperStealth => 7,
            PrimaryRacialTrait::HyperExpansion => 8,
            PrimaryRacialTrait::AlternateReality => 9
        }
    }

    fn advantage_points(&self) -> i16 {
        match *self {
            PrimaryRacialTrait::ClaimAdjuster => 0,
            PrimaryRacialTrait::JackOfAllTrades => 25,
            PrimaryRacialTrait::InterstellarTraveler => -57,
            PrimaryRacialTrait::InnerStrength => 36,
            PrimaryRacialTrait::SpaceDemolition => 53,
            PrimaryRacialTrait::WarMonger => -12,
            PrimaryRacialTrait::PacketPhysics => -37,
            PrimaryRacialTrait::SuperStealth => -28,
            PrimaryRacialTrait::HyperExpansion => -10,
            PrimaryRacialTrait::AlternateReality => -27
        }
    }

    fn techs(&self) -> &[TechnologyId] {
        match *self {
            PrimaryRacialTrait::ClaimAdjuster => CLAIM_ADJUSTER_TECHNOLOGY,
            PrimaryRacialTrait::JackOfAllTrades => JACK_OF_ALL_TRADES_TECHNOLOGY,
            PrimaryRacialTrait::InterstellarTraveler => INTERSTELLAR_TRAVELER_TECHNOLOGY,
            PrimaryRacialTrait::InnerStrength => INNER_STRENGTH_TECHNOLOGY,
            PrimaryRacialTrait::SpaceDemolition => SPACE_DEMOLITION_TECHNOLOGY,
            PrimaryRacialTrait::WarMonger => WAR_MONGER_TECHNOLOGY,
            PrimaryRacialTrait::PacketPhysics => PACKET_PHYSICS_TECHNOLOGY,
            PrimaryRacialTrait::SuperStealth => SUPER_STEALTH_TECHNOLOGY,
            PrimaryRacialTrait::HyperExpansion => HYPER_EXPANSION_TECHNOLOGY,
            PrimaryRacialTrait::AlternateReality => ALTERNATE_REALITY_TECHNOLOGY
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum LesserRacialTrait {
    NoRamscoopEngines,
    ImprovedFuelEfficiency,
    CheapEngines,
    TotalTerraforming,
    OnlyBasicRemoteMining,
    AdvancedRemoteMining,
    NoAdvancedScanners,
    ImprovedStarbases,
    LowStartingPopulation,
    GeneralizedResearch,
    BleedingEdgeTechnology,
    UltimateRecycling,
    RegeneratingShields,
    MineralAlchemy
}

impl LesserRacialTrait {
    fn advantage_points(&self) -> i16 {
        match *self {
            LesserRacialTrait::NoRamscoopEngines => 53,
            LesserRacialTrait::ImprovedFuelEfficiency => -78,
            LesserRacialTrait::CheapEngines => 80,
            LesserRacialTrait::TotalTerraforming => -140,
            LesserRacialTrait::OnlyBasicRemoteMining => 85,
            LesserRacialTrait::AdvancedRemoteMining => -53,
            LesserRacialTrait::NoAdvancedScanners => 95,
            LesserRacialTrait::ImprovedStarbases => -67,
            LesserRacialTrait::LowStartingPopulation => 60,
            LesserRacialTrait::GeneralizedResearch => 13,
            LesserRacialTrait::BleedingEdgeTechnology => 23,
            LesserRacialTrait::UltimateRecycling => -80,
            LesserRacialTrait::RegeneratingShields => 10,
            LesserRacialTrait::MineralAlchemy => -51
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ResearchCost {
    Cheap,
    Normal,
    Expensive
}

#[derive(Serialize, Deserialize)]
pub enum LeftoverPointsOption {
    SurfaceMinerals,
    Mines,
    Factories,
    Defenses,
    MineralConcentration
}

#[derive(Serialize, Deserialize)]
pub struct Race {
    pub name: String,
    pub plural_name: String,
    pub primary_racial_trait: PrimaryRacialTrait,
    pub lesser_racial_traits: Vec<LesserRacialTrait>,

    pub radiation_immune: bool,
    pub radiation_min: u8,
    pub radiation_max: u8,
    pub temperature_immune: bool,
    pub temperature_min: u8,
    pub temperature_max: u8,
    pub gravity_immune: bool,
    pub gravity_min: u8,
    pub gravity_max: u8,

    pub resource_production: u16,
    pub factory_production: u8,
    pub factory_cost: u8,
    pub mine_cost: u8,
    pub mine_production: u8,
    pub colonists_operate_mines: u8,
    pub colonists_operate_factories: u8,
    pub factory_cheap_germanium: bool,
    pub growth_rate: u8,

    pub research_costs: [ResearchCost; 6],
    pub expensive_tech_boost: bool,
    pub leftover_points: LeftoverPointsOption,

    pub icon_index: u32,
    pub advantage_points: i16
}

impl Race {
    pub fn ideal_temperature(&self) -> u8 {
        return f32::round(((self.temperature_min as f32) + (self.temperature_max as f32)) / 2.0) as u8;
    }

    pub fn ideal_radiation(&self) -> u8 {
        return f32::round(((self.radiation_min as f32) + (self.radiation_max as f32)) / 2.0) as u8;

    }

    pub fn ideal_gravity(&self) -> u8 {
        return f32::round(((self.gravity_min as f32) + (self.gravity_max as f32)) / 2.0) as u8;
    }

    pub fn calculate_advantage_points(&self) -> i16 {
        let mut calculated: i16 = 0;
        calculated += self.primary_racial_trait.advantage_points();

        for lrt in self.lesser_racial_traits.iter() {
            calculated += lrt.advantage_points();
        }

        if self.factory_cheap_germanium {
            calculated += -58;
        }

        if self.expensive_tech_boost {
            calculated += 60;
        }

        return calculated;
    }

    pub fn calculate_starting_population(&self, universe_size: UniverseSize) -> u32 {
        let mut population = BASE_STARTING_PLANET_POPULATION;

        if self.primary_racial_trait == PrimaryRacialTrait::InterstellarTraveler && universe_size == UniverseSize::Tiny {
            population -= 5000;
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::LowStartingPopulation) {
            let reduced_population : f32 = population as f32 * 0.7;
            population = f32::round(reduced_population) as u32;
        }

        return population;
    }

    pub fn calculate_starting_tech_levels(&self) -> [u8; 6] {
        let mut tech_levels: [u8; 6] = [0,0,0,0,0,0];

        if self.expensive_tech_boost {
            for x in 0..6 {
                if self.research_costs[x] == ResearchCost::Expensive {
                    tech_levels[x] = 3;
                }
            }
        }

        match self.primary_racial_trait {
            PrimaryRacialTrait::ClaimAdjuster => {
                tech_levels[ResearchField::Biotechnology.value()] = 6;
            },
            PrimaryRacialTrait::JackOfAllTrades => {
                for x in 0..6 {
                    tech_levels[x] = 3;
                }
            },
            PrimaryRacialTrait::InterstellarTraveler => {
                tech_levels[ResearchField::Propulsion.value()] = 5;
                tech_levels[ResearchField::Construction.value()] = 5;
            },
            PrimaryRacialTrait::InnerStrength => {},
            PrimaryRacialTrait::SpaceDemolition => {
                tech_levels[ResearchField::Propulsion.value()] = max(tech_levels[ResearchField::Propulsion.value()], 2);
                tech_levels[ResearchField::Biotechnology.value()] = max(tech_levels[ResearchField::Biotechnology.value()], 2);
            },
            PrimaryRacialTrait::WarMonger => {
                tech_levels[ResearchField::Propulsion.value()] = max(tech_levels[ResearchField::Propulsion.value()], 1);
                tech_levels[ResearchField::Energy.value()] = max(tech_levels[ResearchField::Energy.value()], 1);
                tech_levels[ResearchField::Weapons.value()] = 6;
            },
            PrimaryRacialTrait::PacketPhysics => {
                tech_levels[ResearchField::Energy.value()] = 4;
            },
            PrimaryRacialTrait::SuperStealth => {},
            PrimaryRacialTrait::HyperExpansion => {},
            PrimaryRacialTrait::AlternateReality => {}
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::ImprovedFuelEfficiency) {
            tech_levels[ResearchField::Propulsion.value()] += 1;
        }

        return tech_levels;
    }

    pub fn calculate_initial_available_technologies(&self) -> Vec<TechnologyId> {
        let mut techs = Vec::new();

        for tid in INITIAL_TECHNOLOGY {
            techs.push(tid.clone());
        }

        for tid in self.primary_racial_trait.techs() {
            techs.push(tid.clone());
        }

        if !self.lesser_racial_traits.contains(&LesserRacialTrait::NoAdvancedScanners) {
            for tid in ADVANCED_SHIP_SCANNER_TECHNOLOGY {
                techs.push(tid.clone());
            }

            if self.primary_racial_trait != PrimaryRacialTrait::AlternateReality {
                for tid in ADVANCED_PLANETARY_SCANNER_TECHNOLOGY {
                    techs.push(tid.clone());
                }
            }
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::NoRamscoopEngines) {
            techs.push(TechnologyId::Interspace10);
        }
        else {
            for tid in RAM_SCOOP_ENGINE_TECHNOLOGY {
                techs.push(tid.clone());
            }
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::ImprovedFuelEfficiency) {
            techs.push(TechnologyId::FuelMizer);
            techs.push(TechnologyId::GalaxyScoop);
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::TotalTerraforming) {
            techs.push(TechnologyId::TotalTerraform3);
            techs.push(TechnologyId::TotalTerraform5);
            techs.push(TechnologyId::TotalTerraform7);
            techs.push(TechnologyId::TotalTerraform10);
            techs.push(TechnologyId::TotalTerraform15);
            techs.push(TechnologyId::TotalTerraform20);
            techs.push(TechnologyId::TotalTerraform25);
            techs.push(TechnologyId::TotalTerraform30);
        }

        if !self.lesser_racial_traits.contains(&LesserRacialTrait::OnlyBasicRemoteMining) {
            for tid in NORMAL_REMOTE_MINING_TECHNOLOGY {
                techs.push(tid.clone());
            }
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::AdvancedRemoteMining) {
            techs.push(TechnologyId::RoboMidgetMiner);
            techs.push(TechnologyId::RoboUltraMiner);
            techs.push(TechnologyId::MidgetMiner);
            techs.push(TechnologyId::Miner);
            techs.push(TechnologyId::UltraMiner);
        }

        if self.lesser_racial_traits.contains(&LesserRacialTrait::ImprovedStarbases) {
            techs.push(TechnologyId::SpaceDock);
            techs.push(TechnologyId::UltraStation);
        }

        return techs;
    }

    pub fn generate_initial_ships(&self, universe: &Universe, homeworld: &Planet) {
        match self.primary_racial_trait {
            PrimaryRacialTrait::ClaimAdjuster => {},
            PrimaryRacialTrait::JackOfAllTrades => {},
            PrimaryRacialTrait::InterstellarTraveler => {},
            PrimaryRacialTrait::InnerStrength => {},
            PrimaryRacialTrait::SpaceDemolition => {},
            PrimaryRacialTrait::WarMonger => {},
            PrimaryRacialTrait::PacketPhysics => {},
            PrimaryRacialTrait::SuperStealth => {},
            PrimaryRacialTrait::HyperExpansion => {},
            PrimaryRacialTrait::AlternateReality => {}
        }

    }
}
