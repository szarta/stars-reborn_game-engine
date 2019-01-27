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

/*
 * AI Race Specs built from:
 * https://wiki.starsautohost.org/wiki/AI_Race_Specs_by_Wumpus_-_27_Feb_2007_v2.6/7
 * https://sites.google.com/site/daswumpus/stars_ai
 *
 * verified in original game using race password: viewai
 */
use rand::Rng;

use ::game::objects::race::Race;
use ::game::objects::race::PrimaryRacialTrait;
use ::game::objects::race::LesserRacialTrait;
use ::game::objects::race::ResearchCost;
use ::game::objects::race::LeftoverPointsOption;
use ::game::objects::planet::gravity_display_level_to_habitat_level;
use ::game::objects::planet::temperature_display_level_to_habitat_level;
use ::game::objects::planet::radiation_display_level_to_habitat_level;
use ::game::objects::tech::ResearchField;

pub const CPU_RACE_NAMES: &'static [(&'static str, &'static str)] = &[
    ("American", "Americans"),
    ("Berserker", "Berserkers"),
    ("Bulushi", "Bulushis"),
    ("Cleaver", "Cleavers"),
    ("Crusher", "Crushers"),
    ("Eagle", "Eagles"),
    ("Ferret", "Ferrets"),
    ("Golem", "Golems"),
    ("Hawk", "Hawks"),
    ("Hicardi", "Harcardis"),
    ("Hooveron", "Hooverons"),
    ("House Cat", "House Cats"),
    ("Ubert", "Uberts"),
    ("Kurkonian", "Kurkonians"),
    ("Loraxoid", "Loraxoids"),
    ("Mensoid", "Mensoids"),
    ("Nee", "Nees"),
    ("Valadiac", "Valadiacs")
];

pub fn generate_random_race_name() -> (&'static str, &'static str) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, CPU_RACE_NAMES.len());
    return CPU_RACE_NAMES[index];
}

pub enum CPUDifficulty {
    Expert,
    Tough,
    Standard,
    Easy
}

pub enum PredefinedCPURace {
    Robotoids,
    Turindrones,
    Automitrons,
    Robotils,
    Cybertrons,
    Macinti
}

impl PredefinedCPURace {
    pub fn generate(&self, difficulty: CPUDifficulty) -> Race {
        match *self {
            PredefinedCPURace::Robotoids => create_he(difficulty),
            PredefinedCPURace::Turindrones => create_ss(difficulty),
            PredefinedCPURace::Automitrons => create_is(difficulty),
            PredefinedCPURace::Robotils => create_ca(difficulty),
            PredefinedCPURace::Cybertrons => create_pp(difficulty),
            PredefinedCPURace::Macinti => create_ar(difficulty)
        }
    }
}

pub enum PredefinedRace {
    Antethereal,
    Humanoid,
    Insectoid,
    Nucleotid,
    Rabbitoid,
    Silicanoid
}

impl PredefinedRace {
    pub fn generate(&self) -> Race {
        match *self {
            PredefinedRace::Antethereal => create_antethereal(),
            PredefinedRace::Humanoid => create_humanoid(),
            PredefinedRace::Insectoid => create_insectoid(),
            PredefinedRace::Nucleotid => create_nucleotid(),
            PredefinedRace::Rabbitoid => create_rabbitoid(),
            PredefinedRace::Silicanoid => create_silicanoid()
        }
    }
}

pub fn create_antethereal() -> Race {
    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::NoRamscoopEngines);
    lrt.push(LesserRacialTrait::CheapEngines);
    lrt.push(LesserRacialTrait::AdvancedRemoteMining);
    lrt.push(LesserRacialTrait::NoAdvancedScanners);
    lrt.push(LesserRacialTrait::MineralAlchemy);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap
    ];

    research_costs[ResearchField::Weapons.value()] = ResearchCost::Expensive;

    Race {
        name: "Antethereal".to_string(),
        plural_name: "Antethereals".to_string(),
        primary_racial_trait: PrimaryRacialTrait::SpaceDemolition,
        lesser_racial_traits: lrt,
        gravity_immune: false,
        gravity_min: gravity_display_level_to_habitat_level("0.12"),
        gravity_max: gravity_display_level_to_habitat_level("0.55"),

        temperature_immune: false,
        temperature_min: temperature_display_level_to_habitat_level("-200"),
        temperature_max: temperature_display_level_to_habitat_level("200"),

        radiation_min: radiation_display_level_to_habitat_level("70"),
        radiation_max: radiation_display_level_to_habitat_level("100"),

        radiation_immune: false,

        resource_production: 700,
        factory_production: 11,
        factory_cost: 10,
        mine_cost: 10,
        mine_production: 10,
        colonists_operate_mines: 10,
        colonists_operate_factories: 18,
        factory_cheap_germanium: false,
        growth_rate: 7,

        research_costs: research_costs,
        expensive_tech_boost: false,
        leftover_points: LeftoverPointsOption::SurfaceMinerals,
        advantage_points: 0,
        icon_index: 12,
    }
}
pub fn create_humanoid() -> Race {
    let lrt: Vec<LesserRacialTrait> = Vec::new();
    let research_costs: [ResearchCost; 6] = [
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal
    ];

    Race {
        name: "Humanoid".to_string(),
        plural_name: "Humanoids".to_string(),
        primary_racial_trait: PrimaryRacialTrait::JackOfAllTrades,
        lesser_racial_traits: lrt,
        gravity_immune: false,
        gravity_min: gravity_display_level_to_habitat_level("0.22"),
        gravity_max: gravity_display_level_to_habitat_level("4.40"),

        temperature_immune: false,
        temperature_min: temperature_display_level_to_habitat_level("-140"),
        temperature_max: temperature_display_level_to_habitat_level("140"),

        radiation_immune: false,
        radiation_min: radiation_display_level_to_habitat_level("15"),
        radiation_max: radiation_display_level_to_habitat_level("85"),

        resource_production: 1000,
        factory_production: 10,
        factory_cost: 10,
        mine_cost: 5,
        mine_production: 10,
        colonists_operate_mines: 10,
        colonists_operate_factories: 10,
        factory_cheap_germanium: false,
        growth_rate: 15,

        research_costs: research_costs,
        expensive_tech_boost: false,
        leftover_points: LeftoverPointsOption::SurfaceMinerals,
        advantage_points: 0,
        icon_index: 15,
    }
}

pub fn create_insectoid() -> Race {
    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::CheapEngines);
    lrt.push(LesserRacialTrait::ImprovedStarbases);
    lrt.push(LesserRacialTrait::RegeneratingShields);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap,
        ResearchCost::Cheap
    ];

    research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;
    research_costs[ResearchField::Electronics.value()] = ResearchCost::Normal;

    Race {
        name: "Insectoid".to_string(),
        plural_name: "Insectoids".to_string(),
        primary_racial_trait: PrimaryRacialTrait::WarMonger,
        lesser_racial_traits: lrt,
        gravity_immune: true,
        gravity_min: gravity_display_level_to_habitat_level("0.12"),
        gravity_max: gravity_display_level_to_habitat_level("0.55"),

        temperature_immune: false,
        temperature_min: temperature_display_level_to_habitat_level("-200"),
        temperature_max: temperature_display_level_to_habitat_level("200"),

        radiation_immune: false,
        radiation_min: radiation_display_level_to_habitat_level("70"),
        radiation_max: radiation_display_level_to_habitat_level("100"),

        resource_production: 1000,
        factory_production: 10,
        factory_cost: 10,
        mine_cost: 10,
        mine_production: 9,
        colonists_operate_mines: 6,
        colonists_operate_factories: 10,
        factory_cheap_germanium: false,
        growth_rate: 10,

        research_costs: research_costs,
        expensive_tech_boost: false,
        leftover_points: LeftoverPointsOption::MineralConcentration,
        advantage_points: 0,
        icon_index: 7,
    }
}

pub fn create_nucleotid() -> Race {
    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::AdvancedRemoteMining);
    lrt.push(LesserRacialTrait::ImprovedStarbases);

    let research_costs: [ResearchCost; 6] = [
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive
    ];

    Race {
        name: "Nucleotid".to_string(),
        plural_name: "Nucleotids".to_string(),
        primary_racial_trait: PrimaryRacialTrait::SuperStealth,
        lesser_racial_traits: lrt,
        gravity_immune: true,
        gravity_min: gravity_display_level_to_habitat_level("0.12"),
        gravity_max: gravity_display_level_to_habitat_level("0.55"),

        temperature_immune: false,
        temperature_min: temperature_display_level_to_habitat_level("-152"),
        temperature_max: temperature_display_level_to_habitat_level("152"),

        radiation_immune: false,
        radiation_min: radiation_display_level_to_habitat_level("0"),
        radiation_max: radiation_display_level_to_habitat_level("100"),

        resource_production: 900,
        factory_production: 10,
        factory_cost: 10,
        mine_cost: 15,
        mine_production: 10,
        colonists_operate_mines: 5,
        colonists_operate_factories: 10,
        factory_cheap_germanium: false,
        growth_rate: 10,

        research_costs: research_costs,
        expensive_tech_boost: true,
        leftover_points: LeftoverPointsOption::Factories,
        advantage_points: 0,
        icon_index: 0,
    }
}

pub fn create_rabbitoid() -> Race {
    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::ImprovedFuelEfficiency);
    lrt.push(LesserRacialTrait::TotalTerraforming);
    lrt.push(LesserRacialTrait::CheapEngines);
    lrt.push(LesserRacialTrait::NoAdvancedScanners);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal
    ];

    research_costs[ResearchField::Energy.value()] = ResearchCost::Expensive;
    research_costs[ResearchField::Propulsion.value()] = ResearchCost::Cheap;
    research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Cheap;
    research_costs[ResearchField::Weapons.value()] = ResearchCost::Expensive;

    Race {
        name: "Rabbitoid".to_string(),
        plural_name: "Rabbitoids".to_string(),
        primary_racial_trait: PrimaryRacialTrait::InterstellarTraveler,
        lesser_racial_traits: lrt,
        gravity_immune: false,
        gravity_min: gravity_display_level_to_habitat_level("0.17"),
        gravity_max: gravity_display_level_to_habitat_level("1.24"),

        temperature_immune: false,
        temperature_min: temperature_display_level_to_habitat_level("-60"),
        temperature_max: temperature_display_level_to_habitat_level("124"),

        radiation_immune: false,
        radiation_min: radiation_display_level_to_habitat_level("13"),
        radiation_max: radiation_display_level_to_habitat_level("53"),

        resource_production: 1000,
        factory_production: 10,
        factory_cost: 9,
        mine_cost: 9,
        mine_production: 10,
        colonists_operate_mines: 10,
        colonists_operate_factories: 17,
        factory_cheap_germanium: true,
        growth_rate: 20,

        research_costs: research_costs,
        expensive_tech_boost: false,
        leftover_points: LeftoverPointsOption::Defenses,
        advantage_points: 0,
        icon_index: 1,
    }
}

pub fn create_silicanoid() -> Race {
    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::ImprovedFuelEfficiency);
    lrt.push(LesserRacialTrait::UltimateRecycling);
    lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);
    lrt.push(LesserRacialTrait::BleedingEdgeTechnology);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal
    ];

    research_costs[ResearchField::Propulsion.value()] = ResearchCost::Expensive;
    research_costs[ResearchField::Construction.value()] = ResearchCost::Cheap;
    research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

    Race {
        name: "Silicanoid".to_string(),
        plural_name: "Silicanoids".to_string(),
        primary_racial_trait: PrimaryRacialTrait::HyperExpansion,
        lesser_racial_traits: lrt,
        gravity_immune: true,
        gravity_min: gravity_display_level_to_habitat_level("0.17"),
        gravity_max: gravity_display_level_to_habitat_level("1.24"),

        temperature_immune: true,
        temperature_min: temperature_display_level_to_habitat_level("-60"),
        temperature_max: temperature_display_level_to_habitat_level("124"),

        radiation_immune: true,
        radiation_min: radiation_display_level_to_habitat_level("13"),
        radiation_max: radiation_display_level_to_habitat_level("53"),

        resource_production: 800,
        factory_production: 12,
        factory_cost: 12,
        mine_cost: 9,
        mine_production: 10,
        colonists_operate_mines: 10,
        colonists_operate_factories: 15,
        factory_cheap_germanium: true,
        growth_rate: 6,

        research_costs: research_costs,
        expensive_tech_boost: false,
        leftover_points: LeftoverPointsOption::Factories,
        advantage_points: 0,
        icon_index: 3,
    }
}

pub fn create_he(difficulty: CPUDifficulty ) -> Race {
    let (name, plural_name) = generate_random_race_name();

    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::ImprovedFuelEfficiency);
    lrt.push(LesserRacialTrait::MineralAlchemy);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal
    ];

    let factory_production;
    let factory_cost;
    let colonists_operate_factories;
    let mine_cost;
    let mine_production;
    let colonists_operate_mines;
    let mut factory_cheap_germanium = false;
    let mut growth_rate = 5;
    let expensive_tech_boost = false;
    let resource_production;

    match difficulty {
        CPUDifficulty::Expert => {
            lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);

            research_costs[ResearchField::Construction.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Weapons.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

            resource_production = 800;
            factory_production = 13;
            factory_cost = 9;
            colonists_operate_factories = 16;
            mine_production = 10;
            mine_cost = 4;
            colonists_operate_mines = 8;
            factory_cheap_germanium = true;
            growth_rate = 7;
        },
        CPUDifficulty::Tough => {
            lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);

            research_costs[ResearchField::Propulsion.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Construction.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Weapons.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

            resource_production = 800;
            factory_production = 13;
            factory_cost = 9;
            colonists_operate_factories = 18;
            mine_production = 10;
            mine_cost = 4;
            colonists_operate_mines = 12;
            factory_cheap_germanium = true;
            growth_rate = 6;
        },
        CPUDifficulty::Standard => {
            lrt.push(LesserRacialTrait::CheapEngines);
            lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);

            research_costs[ResearchField::Propulsion.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

            resource_production = 900;
            factory_production = 13;
            factory_cost = 9;
            colonists_operate_factories = 16;
            mine_production = 10;
            mine_cost = 4;
            colonists_operate_mines = 11;
            growth_rate = 6;
        },
        CPUDifficulty::Easy => {
            lrt.push(LesserRacialTrait::CheapEngines);
            lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);
            lrt.push(LesserRacialTrait::BleedingEdgeTechnology);

            research_costs[ResearchField::Propulsion.value()] = ResearchCost::Cheap;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

            resource_production = 1000;
            factory_production = 12;
            factory_cost = 10;
            colonists_operate_factories = 16;
            mine_production = 10;
            mine_cost = 5;
            colonists_operate_mines = 10;
        }
    }

    Race {
        name: name.to_string(),
        plural_name: plural_name.to_string(),
        primary_racial_trait: PrimaryRacialTrait::HyperExpansion,
        lesser_racial_traits: lrt,
        gravity_immune: true,
        gravity_min: 0,
        gravity_max: 100,

        temperature_immune: true,
        temperature_min: 0,
        temperature_max: 100,

        radiation_immune: true,
        radiation_min: 0,
        radiation_max: 100,

        resource_production: resource_production,
        factory_production: factory_production,
        factory_cost: factory_cost,
        colonists_operate_factories: colonists_operate_factories,
        mine_cost: mine_cost,
        mine_production: mine_production,
        colonists_operate_mines: colonists_operate_mines,
        factory_cheap_germanium: factory_cheap_germanium,
        growth_rate: growth_rate,

        research_costs: research_costs,
        expensive_tech_boost: expensive_tech_boost,
        leftover_points: LeftoverPointsOption::SurfaceMinerals,
        advantage_points: 0,
        icon_index: 3,
    }
}

pub fn create_ss(difficulty: CPUDifficulty) -> Race {
    let (name, plural_name) = generate_random_race_name();

    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::ImprovedFuelEfficiency);
    lrt.push(LesserRacialTrait::AdvancedRemoteMining);
    lrt.push(LesserRacialTrait::MineralAlchemy);
    lrt.push(LesserRacialTrait::RegeneratingShields);

    let mut research_costs: [ResearchCost; 6] = [
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal,
        ResearchCost::Normal
    ];

    let gravity_min;
    let gravity_max;
    let temp_min;
    let temp_max;
    let mut rad_min = 0;
    let mut rad_max = 0;
    let mut rad_immune = false;
    let factory_production;
    let factory_cost;
    let colonists_operate_factories;
    let mine_cost;
    let mine_production;
    let colonists_operate_mines;
    let mut factory_cheap_germanium = false;
    let mut growth_rate = 14;
    let mut expensive_tech_boost = false;
    let resource_production;

    match difficulty {
        CPUDifficulty::Expert => {
            research_costs[ResearchField::Energy.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Electronics.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Propulsion.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Construction.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Weapons.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;
            expensive_tech_boost = true;

            gravity_min = gravity_display_level_to_habitat_level("0.56");
            gravity_max = gravity_display_level_to_habitat_level("6.32");
            temp_min = temperature_display_level_to_habitat_level("-180");
            temp_max = temperature_display_level_to_habitat_level("12");
            rad_immune = true;

            resource_production = 800;
            factory_production = 15;
            factory_cost = 10;
            colonists_operate_factories = 25;
            mine_production = 10;
            mine_cost = 5;
            colonists_operate_mines = 9;
            factory_cheap_germanium = true;
            growth_rate = 15;
        },
        CPUDifficulty::Tough => {
            research_costs[ResearchField::Energy.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Propulsion.value()] = ResearchCost::Expensive;

            gravity_min = gravity_display_level_to_habitat_level("0.56");
            gravity_max = gravity_display_level_to_habitat_level("6.08");
            temp_min = temperature_display_level_to_habitat_level("-184");
            temp_max = temperature_display_level_to_habitat_level("8");
            rad_min = radiation_display_level_to_habitat_level("30");
            rad_max = radiation_display_level_to_habitat_level("94");

            resource_production = 900;
            factory_production = 11;
            factory_cost = 10;
            colonists_operate_factories = 10;
            mine_production = 10;
            mine_cost = 5;
            colonists_operate_mines = 9;
            factory_cheap_germanium = true;
        },
        CPUDifficulty::Standard => {
            gravity_min = gravity_display_level_to_habitat_level("0.52");
            gravity_max = gravity_display_level_to_habitat_level("6.08");
            temp_min = temperature_display_level_to_habitat_level("-176");
            temp_max = temperature_display_level_to_habitat_level("40");
            rad_min = radiation_display_level_to_habitat_level("26");
            rad_max = radiation_display_level_to_habitat_level("96");

            resource_production = 1000;
            factory_production = 10;
            factory_cost = 10;
            colonists_operate_factories = 10;
            mine_production = 10;
            mine_cost = 5;
            colonists_operate_mines = 9;
            factory_cheap_germanium = true;
        },
        CPUDifficulty::Easy => {
            research_costs[ResearchField::Weapons.value()] = ResearchCost::Expensive;
            research_costs[ResearchField::Biotechnology.value()] = ResearchCost::Expensive;

            gravity_min = gravity_display_level_to_habitat_level("0.52");
            gravity_max = gravity_display_level_to_habitat_level("5.36");
            temp_min = temperature_display_level_to_habitat_level("-172");
            temp_max = temperature_display_level_to_habitat_level("52");
            rad_min = radiation_display_level_to_habitat_level("35");
            rad_max = radiation_display_level_to_habitat_level("95");

            resource_production = 1000;
            factory_production = 9;
            factory_cost = 10;
            colonists_operate_factories = 9;
            mine_production = 9;
            mine_cost = 5;
            colonists_operate_mines = 8;
        }
    }

    Race {
        name: name.to_string(),
        plural_name: plural_name.to_string(),
        primary_racial_trait: PrimaryRacialTrait::SuperStealth,
        lesser_racial_traits: lrt,
        gravity_immune: false,
        gravity_min: gravity_min,
        gravity_max: gravity_max,

        temperature_immune: false,
        temperature_min: temp_min,
        temperature_max: temp_max,

        radiation_immune: rad_immune,
        radiation_min: rad_min,
        radiation_max: rad_max,

        resource_production: resource_production,
        factory_production: factory_production,
        factory_cost: factory_cost,
        colonists_operate_factories: colonists_operate_factories,
        mine_cost: mine_cost,
        mine_production: mine_production,
        colonists_operate_mines: colonists_operate_mines,
        factory_cheap_germanium: factory_cheap_germanium,
        growth_rate: growth_rate,

        research_costs: research_costs,
        expensive_tech_boost: expensive_tech_boost,
        leftover_points: LeftoverPointsOption::SurfaceMinerals,
        advantage_points: 0,
        icon_index: 2,
    }
}

pub fn create_is(difficulty: CPUDifficulty) -> Race {
    let (name, plural_name) = generate_random_race_name();

    let mut lrt: Vec<LesserRacialTrait> = Vec::new();
    lrt.push(LesserRacialTrait::GeneralizedResearch);
    lrt.push(LesserRacialTrait::OnlyBasicRemoteMining);
    lrt.push(LesserRacialTrait::NoAdvancedScanners);
    lrt.push(LesserRacialTrait::LowStartingPopulation);

    let research_costs: [ResearchCost; 6] = [
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive,
        ResearchCost::Expensive
    ];

    let gravity_min = gravity_display_level_to_habitat_level("0.15");
    let gravity_max = gravity_display_level_to_habitat_level("1.52");
    let temp_min;
    let temp_max;
    let mut temp_immune = false;
    let rad_min;
    let rad_max;
    let factory_production;
    let factory_cost;
    let colonists_operate_factories;
    let mine_cost;
    let mine_production;
    let colonists_operate_mines;
    let mut factory_cheap_germanium = false;
    let mut growth_rate = 15;
    let expensive_tech_boost = true;
    let resource_production;

    match difficulty {
        CPUDifficulty::Expert => {
            temp_immune = true;
            temp_min = 0;
            temp_max = 100;
            rad_min = radiation_display_level_to_habitat_level("0");
            rad_max = radiation_display_level_to_habitat_level("100");

            resource_production = 800;
            factory_production = 14;
            factory_cost = 9;
            colonists_operate_factories = 14;
            mine_production = 14;
            mine_cost = 5;
            colonists_operate_mines = 14;
            factory_cheap_germanium = true;
            growth_rate = 16;
        },
        CPUDifficulty::Tough => {
            temp_min = temperature_display_level_to_habitat_level("-96");
            temp_max = temperature_display_level_to_habitat_level("176");
            rad_min = radiation_display_level_to_habitat_level("5");
            rad_max = radiation_display_level_to_habitat_level("71");

            resource_production = 800;
            factory_production = 14;
            factory_cost = 9;
            colonists_operate_factories = 15;
            mine_production = 14;
            mine_cost = 5;
            colonists_operate_mines = 15;
            factory_cheap_germanium = true;
        },
        CPUDifficulty::Standard => {
            lrt.push(LesserRacialTrait::CheapEngines);

            temp_min = temperature_display_level_to_habitat_level("-96");
            temp_max = temperature_display_level_to_habitat_level("176");
            rad_min = radiation_display_level_to_habitat_level("5");
            rad_max = radiation_display_level_to_habitat_level("71");

            resource_production = 800;
            factory_production = 13;
            factory_cost = 9;
            colonists_operate_factories = 14;
            mine_production = 10;
            mine_cost = 6;
            colonists_operate_mines = 14;
            factory_cheap_germanium = true;
        },
        CPUDifficulty::Easy => {
            lrt.push(LesserRacialTrait::CheapEngines);

            temp_min = temperature_display_level_to_habitat_level("-96");
            temp_max = temperature_display_level_to_habitat_level("176");
            rad_min = radiation_display_level_to_habitat_level("5");
            rad_max = radiation_display_level_to_habitat_level("71");

            resource_production = 900;
            factory_production = 11;
            factory_cost = 10;
            colonists_operate_factories = 14;
            mine_production = 11;
            mine_cost = 6;
            colonists_operate_mines = 14;
        }
    }

    Race {
        name: name.to_string(),
        plural_name: plural_name.to_string(),
        primary_racial_trait: PrimaryRacialTrait::InnerStrength,
        lesser_racial_traits: lrt,
        gravity_immune: false,
        gravity_min: gravity_min,
        gravity_max: gravity_max,

        temperature_immune: temp_immune,
        temperature_min: temp_min,
        temperature_max: temp_max,

        radiation_immune: false,
        radiation_min: rad_min,
        radiation_max: rad_max,

        resource_production: resource_production,
        factory_production: factory_production,
        factory_cost: factory_cost,
        colonists_operate_factories: colonists_operate_factories,
        mine_cost: mine_cost,
        mine_production: mine_production,
        colonists_operate_mines: colonists_operate_mines,
        factory_cheap_germanium: factory_cheap_germanium,
        growth_rate: growth_rate,

        research_costs: research_costs,
        expensive_tech_boost: expensive_tech_boost,
        leftover_points: LeftoverPointsOption::MineralConcentration,
        advantage_points: 0,
        icon_index: 2,
    }
}

pub fn create_ca(difficulty: CPUDifficulty) -> Race {
    return create_he(difficulty);
}

pub fn create_pp(difficulty: CPUDifficulty) -> Race {
    return create_he(difficulty);
}

pub fn create_ar(difficulty: CPUDifficulty) -> Race {
    return create_he(difficulty);
}
