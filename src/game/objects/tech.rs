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
const TECHNOLOGY_BASE_COSTS : &'static [u32] = &[
    0,
    50,
    80,
    130,
    210,
    340,
    550,
    890,
    1440,
    2330,
    3770,
    6100,
    9870,
    13850,
    18040,
    22440,
    27050,
    31870,
    36900,
    42140,
    47590,
    53250,
    59120,
    65200,
    71490,
    77990,
    84700
];

/*
    Calculate the total cost for a technology to get to the next level.

    The base cost is provided in a table.

    The base cost is, then, modified by 3 factors:

        slow_tech - boolean - whether game parameter was selected to
                              slow the tech advances

        total_levels - the number of total tech levels attained so far
        cost_percent - the cost selected in race edit for the technology
            one of: 50, 100, 150

    total = (base + (tech_levels * 10)) * cost_percent

    Double the total if slow_tech_advance is True.

    Based on algorithm from:
    http://wiki.starsautohost.org/wiki/Guts_of_research_costs

    Credit to S.B. Posey's Spreadsheet used to double-check the numbers.
*/
pub fn calculate_cost_to_next_tech_level(current_level : u8, total_levels : u16, cost_percent : u8, slow_tech : bool) -> f32 {
    let next_level : usize = (current_level as usize) + 1;
    let mut cost : f32 = 
        ((TECHNOLOGY_BASE_COSTS[next_level] + ((total_levels as u32) * 10)) as f32) * ((cost_percent as f32) / 100.0);


    if slow_tech {
        cost *= 2.0;
    }

    return cost;
}

#[derive(Serialize, Deserialize)]
pub struct TechnologyCost {
    pub ironium: u16,
    pub boranium: u16,
    pub germanium: u16,
    pub resources: u16,
}

#[derive(Serialize, Deserialize)]
pub enum MineType {
    Normal,
    Heavy,
    Speed
}

#[derive(Serialize, Deserialize)]
pub struct TechnologyRequirement {
    pub levels: [u8; 6]
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TechnologyId {
    Viewer50 = 0,
    Viewer90 = 1,
    Scoper150 = 2,
    Scoper220 = 3,
    Scoper280 = 4,
    Snooper320X = 5,
    Snooper400X = 6,
    Snooper500X = 7,
    Snooper620X = 8,
    SDI = 9,
    MissileBattery = 10,
    LaserBattery = 11,
    PlanetaryShield = 12,
    NeutronShield = 13,
    Tritanium = 14,
    Crobmnium = 15,
    CarbonicArmor = 16,
    Strobnium = 17,
    OrganicArmor = 18,
    Kelarium = 19,
    FieldedKelarium = 20,
    DepletedNeutronium = 21,
    Neutronium = 22,
    Valanium = 23,
    Superlatanium = 24,
    MoleskinShield = 25,
    CowhideShield = 26,
    WolverineDiffuseShield = 27,
    CrobySharmor = 28,
    ShadowShield = 29,
    BearNeutrinoBarrier = 30,
    GorillaDelagator = 31,
    ElephantHideFortress = 32,
    CompletePhaseShield = 33,
    MineDispenser40 = 34,
    MineDispenser50 = 35,
    MineDispenser80 = 36,
    MineDispenser130 = 37,
    HeavyDispenser50 = 38,
    HeavyDispenser110 = 39,
    HeavyDispenser200 = 40,
    SpeedTrap20 = 41,
    SpeedTrap30 = 42,
    SpeedTrap50 = 43,
    BatScanner = 44,
    RhinoScanner = 45,
    MoleScanner = 46,
    DNAScanner = 47,
    PossumScanner = 48,
    PickPocketScanner = 49,
    ChameleonScanner = 50,
    FerretScanner = 51,
    DolphinScanner = 52,
    GazelleScanner = 53,
    RNAScanner = 54,
    CheetahScanner = 55,
    ElephantScanner = 56,
    EagleEyeScanner = 57,
    RobberBaronScanner = 58,
    PeerlessScanner = 59,
    ColonizationModule = 60,
    OrbitalConstructionModule = 61,
    CargoPod = 62,
    SuperCargoPod = 63,
    FuelTank = 64,
    SuperFuelTank = 65,
    ManeuveringJet = 66,
    Overthruster = 67,
    BeamDeflector = 68,
    TransportCloaking = 69,
    StealthCloak = 70,
    SuperStealthCloak = 71,
    UltraStealthCloak = 72,
    BattleComputer = 73,
    BattleSuperComputer = 74,
    BattleNexus = 75,
    Jammer10 = 76,
    Jammer20 = 77,
    Jammer30 = 78,
    Jammer50 = 79,
    EnergyCapacitor = 80,
    FluxCapacitor = 81,
    EnergyDampener = 82,
    TachyonDetector = 83,
    AntiMatterGenerator = 84,
    TotalTerraform3 = 85,
    TotalTerraform5 = 86,
    TotalTerraform7 = 87,
    TotalTerraform10 = 88,
    TotalTerraform15 = 89,
    TotalTerraform20 = 90,
    TotalTerraform25 = 91,
    TotalTerraform30 = 92,
    GravityTerraform3 = 93,
    GravityTerraform7 = 94,
    GravityTerraform11 = 95,
    GravityTerraform15 = 96,
    TemperatureTerraform3 = 97,
    TemperatureTerraform7 = 98,
    TemperatureTerraform11 = 99,
    TemperatureTerraform15 = 100,
    RadiationTerraform3 = 101,
    RadiationTerraform7 = 102,
    RadiationTerraform11 = 103,
    RadiationTerraform15 = 104,
    RoboMidgetMiner = 105,
    RoboMiniMiner = 106,
    RoboMiner = 107,
    RoboMaxiMiner = 108,
    RoboSuperMiner = 109,
    RoboUltraMiner = 110,
    OrbitalAdjuster = 111,
    SettlersDelight = 112,
    QuickJump5 = 113,
    FuelMizer = 114,
    LongHump6 = 115,
    DaddyLongLegs7 = 116,
    AlphaDrive8 = 117,
    TransGalacticDrive = 118,
    Interspace10 = 119,
    TransStar10 = 120,
    RadiatingHydroRamScoop = 121,
    SubGalacticFuelScoop = 122,
    TransGalacticFuelScoop = 123,
    TransGalacticSuperScoop = 124,
    TransGalacticMizerScoop = 125,
    GalaxyScoop = 126,
    LadyFingerBomb = 127,
    BlackCatBomb = 128,
    M70Bomb = 129,
    M80Bomb = 130,
    CherryBomb = 131,
    LBU17Bomb = 132,
    LBU32Bomb = 133,
    LBU74Bomb = 134,
    RetroBomb = 135,
    SmartBomb = 136,
    NeutronBomb = 137,
    EnrichedNeutronBomb = 138,
    PeerlessBomb = 139,
    AnnihilatorBomb = 140,
    Stargate100_250 = 141,
    StargateAny_300 = 142,
    Stargate150_600 = 143,
    Stargate300_500 = 144,
    Stargate100_Any = 145,
    StargateAny_800 = 146,
    StargateAny_Any = 147,
    MassDriver5 = 148,
    MassDriver6 = 149,
    MassDriver7 = 150,
    SuperDriver8 = 151,
    SuperDriver9 = 152,
    UltraDriver10 = 153,
    UltraDriver11 = 154,
    UltraDriver12 = 155,
    UltraDriver13 = 156,
    OrbitalFort = 157,
    SpaceDock = 158,
    SpaceStation = 159,
    UltraStation = 160,
    DeathStar = 161,
    SmallFreighter = 162,
    MediumFreighter = 163,
    LargeFreighter = 164,
    SuperFreighter = 165,
    Scout = 166,
    Frigate = 167,
    Destroyer = 168,
    Cruiser = 169,
    BattleCruiser = 170,
    Battleship = 171,
    Dreadnought = 172,
    Privateer = 173,
    Rogue = 174,
    Galleon = 175,
    MiniColonyShip = 176,
    ColonyShip = 177,
    MiniBomber = 178,
    B17Bomber = 179,
    StealthBomber = 180,
    B52Bomber = 181,
    MidgetMiner = 182,
    MiniMiner = 183,
    Miner = 184,
    MaxiMiner = 185,
    UltraMiner = 186,
    FuelTransport = 187,
    SuperFuelTransport = 188,
    MiniMineLayer = 189,
    SuperMineLayer = 190,
    Nubian = 191,
    MetaMorph = 192,
    Laser = 193,
    XrayLaser = 194,
    MiniGun = 195,
    YakimoraLightPhaser = 196,
    Blackjack = 197,
    PhaserBazooka = 198,
    PulsedSapper = 199,
    ColloidalPhaser = 200,
    GatlingGun = 201,
    MiniBlaster = 202,
    Bludgeon = 203,
    MarkIVBlaster = 204,
    PhasedSapper = 205,
    HeavyBlaster = 206,
    GatlingNeutrinoCannon = 207,
    MyopicDisruptor = 208,
    Blunderbuss = 209,
    Disruptor = 210,
    SyncroSapper = 211,
    MegaDisruptor = 212,
    BigMuthaCannon = 213,
    StreamingPulverizer = 214,
    AntiMatterPulverizer = 215,
    AlphaTorpedo = 216,
    BetaTorpedo = 217,
    DeltaTorpedo = 218,
    EpsilonTorpedo = 219,
    RhoTorpedo = 220,
    UpsilonTorpedo = 221,
    OmegaTorpedo = 222,
    JihadMissile = 223,
    JuggernautMissile = 224,
    DoomsdayMissile = 225,
    ArmageddonMissile = 226,
    Hushaboom = 227,
    EnigmaPulsar = 228,
    MegaPolyShell = 229,
    LangstonShell = 230,
    MultiFunctionPod = 231,
    AntiMatterTorpedo = 232,
    JumpGate = 233,
    GenesisDevice = 234,
    MultiContainedMunition = 235,
    AlienMiner = 236,
    MultiCargoPod = 237,
    MiniMorph = 238
}

#[derive(Serialize, Deserialize)]
pub struct Technology {
    pub requirement: TechnologyRequirement,
    pub cost: TechnologyCost,
    pub basic_range: Option<u16>,
    pub penetrating_range: Option<u16>,
    pub coverage: Option<f32>,
    pub mass: Option<u32>,
    pub armor: Option<u32>,
    pub shield_value: Option<u32>,
    pub cloaking: Option<u8>,
    pub mines_per_year: Option<u32>,
    pub mine_type: Option<MineType>,
    pub cargo: Option<u32>,
    pub fuel: Option<u32>,
    pub fuel_per_year: Option<u32>,
    pub battle_speed_modifier: Option<f32>,
    pub beam_reduction: Option<u8>,
    pub torpedo_accuracy: Option<u8>,
    pub jamming: Option<u8>,
    pub beam_damage: Option<u8>,
    pub mining_value: Option<u16>,
    pub terraforming_temperature: Option<u8>,
    pub terraforming_gravity: Option<u8>,
    pub terraforming_radiation: Option<u8>,
    pub battle_speed: Option<u8>,
    pub warp10_travel: Option<bool>,
    pub fuel_table: Option<[u16; 11]>,
    pub is_smart: Option<bool>,
    pub colonist_kill_percent: Option<f32>,
    pub min_colonists_killed: Option<u16>,
    pub buildings_destroyed: Option<u16>,
    pub power: Option<u16>,
    pub range: Option<u8>,
    pub initiative: Option<u8>,
    pub is_spread: Option<bool>,
    pub hits_shields_only: Option<bool>,
    pub accuracy: Option<u8>,
    pub safe_mass: Option<u16>,
    pub safe_distance: Option<u16>,
    pub warp: Option<u8>
}

pub const TECHNOLOGY_DETAILS: &'static [Technology] = &[
    Technology { // Viewer 50
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(50),
        penetrating_range: Some(0),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Viewer 90
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,1,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(90),
        penetrating_range: Some(0),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper150
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,3,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(150),
        penetrating_range: Some(0),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper220
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,6,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(220),
        penetrating_range: Some(0),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper280
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,8,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(280),
        penetrating_range: Some(0),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper320X
        requirement: TechnologyRequirement {
            levels: [3,0,0,0,10,3]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(320),
        penetrating_range: Some(160),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper400X
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,13,6]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(400),
        penetrating_range: Some(200),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper500X
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,16,7]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(500),
        penetrating_range: Some(250),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Scoper620X
        requirement: TechnologyRequirement {
            levels: [7,0,0,0,23,9]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 70,
            resources: 100
        },
        basic_range: Some(620),
        penetrating_range: Some(310),
        coverage: None, mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // SDI
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 5,
            resources: 15
        },
        coverage: Some(0.0099),
        basic_range: None, penetrating_range: None,
        mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Missile Battery
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 5,
            resources: 15
        },
        coverage: Some(0.0199),
        basic_range: None, penetrating_range: None,
        mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Laser Battery
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 5,
            resources: 15
        },
        coverage: Some(0.0239),
        basic_range: None, penetrating_range: None,
        mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Planetary Shield
        requirement: TechnologyRequirement {
            levels: [16,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 5,
            resources: 15
        },
        coverage: Some(0.0299),
        basic_range: None, penetrating_range: None,
        mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Neutron Shield
        requirement: TechnologyRequirement {
            levels: [23,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 5,
            resources: 15
        },
        coverage: Some(0.0379),
        basic_range: None, penetrating_range: None,
        mass: None, armor: None, shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Tritanium
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 0,
            resources: 10
        },
        mass: Some(60),
        armor: Some(50),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Crobmnium
        requirement: TechnologyRequirement {
            levels: [0,0,0,3,0,0]
        },
        cost: TechnologyCost {
            ironium: 6,
            boranium: 0,
            germanium: 0,
            resources: 13
        },
        mass: Some(56),
        armor: Some(75),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Carbonic Armor
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,4]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 5,
            resources: 15
        },
        mass: Some(25),
        armor: Some(100),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Strobnium
        requirement: TechnologyRequirement {
            levels: [0,0,0,6,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 0,
            resources: 18
        },
        mass: Some(54),
        armor: Some(120),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Organic Armor
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,7]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 6,
            resources: 20
        },
        mass: Some(15),
        armor: Some(175),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Kelarium
        requirement: TechnologyRequirement {
            levels: [0,0,0,9,0,0]
        },
        cost: TechnologyCost {
            ironium: 9,
            boranium: 1,
            germanium: 0,
            resources: 25
        },
        mass: Some(15),
        armor: Some(175),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Fielded Kelarium
        requirement: TechnologyRequirement {
            levels: [4,0,0,10,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 2,
            resources: 28
        },
        mass: Some(50),
        armor: Some(125),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        cloaking: None, mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Depleted Neutronium
        requirement: TechnologyRequirement {
            levels: [0,0,0,10,3,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 2,
            resources: 28
        },
        mass: Some(50),
        armor: Some(200),
        cloaking: Some(25),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, 
        mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Neutronium
        requirement: TechnologyRequirement {
            levels: [0,0,0,12,0,0]
        },
        cost: TechnologyCost {
            ironium: 11,
            boranium: 2,
            germanium: 1,
            resources: 30
        },
        mass: Some(45),
        armor: Some(275),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Valanium
        requirement: TechnologyRequirement {
            levels: [0,0,0,16,0,0]
        },
        cost: TechnologyCost {
            ironium: 15,
            boranium: 0,
            germanium: 0,
            resources: 50
        },
        mass: Some(40),
        armor: Some(500),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Superlatanium
        requirement: TechnologyRequirement {
            levels: [0,0,0,24,0,0]
        },
        cost: TechnologyCost {
            ironium: 25,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        mass: Some(30),
        armor: Some(1500),
        basic_range: None, penetrating_range: None, coverage: None,
        shield_value: None, cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Moleskin Shield
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 0,
            germanium: 1,
            resources: 4
        },
        mass: Some(1),
        shield_value: Some(25),
        basic_range: None, penetrating_range: None, coverage: None,
        cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Cowhide Shield
        requirement: TechnologyRequirement {
            levels: [3,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 2,
            resources: 5
        },
        mass: Some(1),
        shield_value: Some(40),
        basic_range: None, penetrating_range: None, coverage: None,
        cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Wolverine Diffuse Shield
        requirement: TechnologyRequirement {
            levels: [6,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 3,
            resources: 6
        },
        mass: Some(1),
        shield_value: Some(60),
        basic_range: None, penetrating_range: None, coverage: None,
        cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Croby Sharmor
        requirement: TechnologyRequirement {
            levels: [7,0,0,4,0,0]
        },
        cost: TechnologyCost {
            ironium: 7,
            boranium: 0,
            germanium: 4,
            resources: 15
        },
        mass: Some(10),
        shield_value: Some(60),
        armor: Some(65),
        basic_range: None, penetrating_range: None, coverage: None,
        cloaking: None,
        mines_per_year: None, mine_type: None, cargo: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Shadow Shield
        requirement: TechnologyRequirement {
            levels: [7,0,0,0,3,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 3,
            resources: 7
        },
        mass: Some(2),
        shield_value: Some(75),
        cloaking: Some(35),
        basic_range: None, penetrating_range: None, coverage: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Bear Neutrino Barrier
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 0,
            germanium: 4,
            resources: 8
        },
        mass: Some(1),
        shield_value: Some(100),
        cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Gorilla Delegator
        requirement: TechnologyRequirement {
            levels: [14,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 6,
            resources: 11 
        },
        mass: Some(1),
        shield_value: Some(175),
        cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Elephant Hide Fortress
        requirement: TechnologyRequirement {
            levels: [18,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 10,
            resources: 15 
        },
        mass: Some(1),
        shield_value: Some(300),
        cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Complete Phase Shield
        requirement: TechnologyRequirement {
            levels: [22,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 0,
            germanium: 15,
            resources: 20 
        },
        mass: Some(1),
        shield_value: Some(500),
        cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        mines_per_year: None, mine_type: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Mine Dispenser 40
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 10,
            germanium: 8,
            resources: 45 
        },
        mass: Some(25),
        mines_per_year: Some(40),
        mine_type: Some(MineType::Normal),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Mine Dispenser 50
        requirement: TechnologyRequirement {
            levels: [2,0,0,0,0,4]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 12,
            germanium: 10,
            resources: 55 
        },
        mass: Some(30),
        mines_per_year: Some(50),
        mine_type: Some(MineType::Normal),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Mine Dispenser 80
        requirement: TechnologyRequirement {
            levels: [3,0,0,0,0,7]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 14,
            germanium: 10,
            resources: 65 
        },
        mass: Some(30),
        mines_per_year: Some(80),
        mine_type: Some(MineType::Normal),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Mine Dispenser 130
        requirement: TechnologyRequirement {
            levels: [6,0,0,0,0,12]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 18,
            germanium: 10,
            resources: 80 
        },
        mass: Some(30),
        mines_per_year: Some(130),
        mine_type: Some(MineType::Normal),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Heavy Dispenser 50
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,0,3]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 20,
            germanium: 5,
            resources: 50 
        },
        mass: Some(10),
        mines_per_year: Some(50),
        mine_type: Some(MineType::Heavy),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Heavy Dispenser 110
        requirement: TechnologyRequirement {
            levels: [9,0,0,0,0,5]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 30,
            germanium: 5,
            resources: 70 
        },
        mass: Some(15),
        mines_per_year: Some(110),
        mine_type: Some(MineType::Heavy),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Heavy Dispenser 200
        requirement: TechnologyRequirement {
            levels: [14,0,0,0,0,7]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 45,
            germanium: 5,
            resources: 90 
        },
        mass: Some(20),
        mines_per_year: Some(200),
        mine_type: Some(MineType::Heavy),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Speed Trap 20
        requirement: TechnologyRequirement {
            levels: [0,0,2,0,0,2]
        },
        cost: TechnologyCost {
            ironium: 29,
            boranium: 0,
            germanium: 12,
            resources: 58 
        },
        mass: Some(100),
        mines_per_year: Some(20),
        mine_type: Some(MineType::Speed),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Speed Trap 30
        requirement: TechnologyRequirement {
            levels: [0,0,3,0,0,6]
        },
        cost: TechnologyCost {
            ironium: 32,
            boranium: 0,
            germanium: 14,
            resources: 72 
        },
        mass: Some(135),
        mines_per_year: Some(30),
        mine_type: Some(MineType::Speed),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Speed Trap 50
        requirement: TechnologyRequirement {
            levels: [0,0,5,0,0,11]
        },
        cost: TechnologyCost {
            ironium: 40,
            boranium: 0,
            germanium: 15,
            resources: 80 
        },
        mass: Some(140),
        mines_per_year: Some(50),
        mine_type: Some(MineType::Speed),
        shield_value: None, cloaking: None,
        basic_range: None, penetrating_range: None, coverage: None,
        cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Bat Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 0,
            germanium: 1,
            resources: 1 
        },
        mass: Some(2),
        basic_range: Some(0), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Rhino Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,1,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 2,
            resources: 3 
        },
        mass: Some(5),
        basic_range: Some(50), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Mole Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,4,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 2,
            resources: 9 
        },
        mass: Some(2),
        basic_range: Some(100), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // DNA Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,3,0,0,6]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 1,
            germanium: 1,
            resources: 5 
        },
        mass: Some(2),
        basic_range: Some(125), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Possum Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,5,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 3,
            resources: 18
        },
        mass: Some(3),
        basic_range: Some(150), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // PickPocket Scanner
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,4,4]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 10,
            germanium: 6,
            resources: 35
        },
        mass: Some(15),
        basic_range: Some(80), 
        penetrating_range: Some(0), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Chameleon Scanner
        requirement: TechnologyRequirement {
            levels: [3,0,0,0,6,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 6,
            germanium: 4,
            resources: 25
        },
        mass: Some(6),
        basic_range: Some(160), 
        penetrating_range: Some(45), 
        cloaking: Some(20),
        mines_per_year: None, mine_type: None, shield_value: None, 
        coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Ferret Scanner
        requirement: TechnologyRequirement {
            levels: [3,0,0,0,7,2]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 8,
            resources: 36
        },
        mass: Some(2),
        basic_range: Some(185), 
        penetrating_range: Some(50), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    },
    Technology { // Dolphin Scanner
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,10,4]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 5,
            germanium: 10,
            resources: 40
        },
        mass: Some(4),
        basic_range: Some(220), 
        penetrating_range: Some(100), 
        mines_per_year: None, mine_type: None, shield_value: None, 
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None, 
        beam_reduction: None, torpedo_accuracy: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None, 
        is_smart: None, colonist_kill_percent: None, 
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None, 
        hits_shields_only: None, accuracy: None, safe_mass: None, 
        safe_distance: None, warp: None
    }
];

// Everyone gets these technologies
pub const INITIAL_TECHNOLOGY : &'static [TechnologyId] = &[
    TechnologyId::Tritanium,
    TechnologyId::Laser,
    TechnologyId::QuickJump5,
    TechnologyId::BattleComputer,
    TechnologyId::FuelTank,
    TechnologyId::BatScanner,
    TechnologyId::MoleskinShield,
    TechnologyId::SmallFreighter,
    TechnologyId::Scout,
    TechnologyId::ColonyShip,
    TechnologyId::OrbitalFort,
    TechnologyId::SpaceStation,
    TechnologyId::AlphaTorpedo
];

pub const BASE_DISCOVERABLE_TECHNOLOGY : &'static [TechnologyId] = &[
    TechnologyId::Crobmnium,
    TechnologyId::CarbonicArmor,
    TechnologyId::Strobnium,
    TechnologyId::OrganicArmor,
    TechnologyId::Kelarium,
    TechnologyId::Neutronium,
    TechnologyId::Valanium,
    TechnologyId::Superlatanium,
    TechnologyId::XrayLaser,
    TechnologyId::YakimoraLightPhaser,
    TechnologyId::Blackjack,
    TechnologyId::PhaserBazooka,
    TechnologyId::PulsedSapper,
    TechnologyId::ColloidalPhaser,
    TechnologyId::GatlingGun,
    TechnologyId::MiniBlaster,
    TechnologyId::Bludgeon,
    TechnologyId::MarkIVBlaster,
    TechnologyId::PhasedSapper,
    TechnologyId::HeavyBlaster,
    TechnologyId::MyopicDisruptor,
    TechnologyId::Disruptor,
    TechnologyId::SyncroSapper,
    TechnologyId::MegaDisruptor,
    TechnologyId::BigMuthaCannon,
    TechnologyId::StreamingPulverizer,
    TechnologyId::AntiMatterPulverizer,
    TechnologyId::LadyFingerBomb,
    TechnologyId::BlackCatBomb,
    TechnologyId::M70Bomb,
    TechnologyId::M80Bomb,
    TechnologyId::CherryBomb,
    TechnologyId::LBU17Bomb,
    TechnologyId::LBU32Bomb,
    TechnologyId::LBU74Bomb,
    TechnologyId::StealthCloak,
    TechnologyId::SuperStealthCloak,
    TechnologyId::BattleSuperComputer,
    TechnologyId::BattleNexus,
    TechnologyId::Jammer20,
    TechnologyId::Jammer30,
    TechnologyId::EnergyCapacitor,
    TechnologyId::LongHump6,
    TechnologyId::DaddyLongLegs7,
    TechnologyId::AlphaDrive8,
    TechnologyId::TransGalacticDrive,
    TechnologyId::TransStar10,
    TechnologyId::RadiatingHydroRamScoop,
    TechnologyId::CargoPod,
    TechnologyId::SuperCargoPod,
    TechnologyId::SuperFuelTank,
    TechnologyId::ManeuveringJet,
    TechnologyId::Overthruster,
    TechnologyId::BeamDeflector,
    TechnologyId::RoboMiniMiner,
    TechnologyId::MassDriver7,
    TechnologyId::UltraDriver10,
    TechnologyId::RhinoScanner,
    TechnologyId::MoleScanner,
    TechnologyId::DNAScanner,
    TechnologyId::PossumScanner,
    TechnologyId::GazelleScanner,
    TechnologyId::RNAScanner,
    TechnologyId::CheetahScanner,
    TechnologyId::EagleEyeScanner,
    TechnologyId::PeerlessScanner,
    TechnologyId::CowhideShield,
    TechnologyId::WolverineDiffuseShield,
    TechnologyId::BearNeutrinoBarrier,
    TechnologyId::GorillaDelagator,
    TechnologyId::ElephantHideFortress,
    TechnologyId::CompletePhaseShield,
    TechnologyId::MediumFreighter,
    TechnologyId::LargeFreighter,
    TechnologyId::Frigate,
    TechnologyId::Destroyer,
    TechnologyId::Cruiser,
    TechnologyId::Battleship,
    TechnologyId::Privateer,
    TechnologyId::Galleon,
    TechnologyId::MiniBomber,
    TechnologyId::B17Bomber,
    TechnologyId::MiniMiner,
    TechnologyId::SuperFuelTransport,
    TechnologyId::Nubian,
    TechnologyId::GravityTerraform3,
    TechnologyId::GravityTerraform7,
    TechnologyId::GravityTerraform11,
    TechnologyId::GravityTerraform15,
    TechnologyId::TemperatureTerraform3,
    TechnologyId::TemperatureTerraform7,
    TechnologyId::TemperatureTerraform11,
    TechnologyId::TemperatureTerraform15,
    TechnologyId::RadiationTerraform3,
    TechnologyId::RadiationTerraform7,
    TechnologyId::RadiationTerraform11,
    TechnologyId::RadiationTerraform15,
    TechnologyId::BetaTorpedo,
    TechnologyId::DeltaTorpedo,
    TechnologyId::EpsilonTorpedo,
    TechnologyId::RhoTorpedo,
    TechnologyId::UpsilonTorpedo,
    TechnologyId::OmegaTorpedo,
    TechnologyId::JihadMissile,
    TechnologyId::JuggernautMissile,
    TechnologyId::DoomsdayMissile,
    TechnologyId::ArmageddonMissile
];

pub const NORMAL_REMOTE_MINING_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::RoboMiner,
    TechnologyId::RoboMaxiMiner,
    TechnologyId::RoboSuperMiner,
    TechnologyId::MaxiMiner
];

pub const RAM_SCOOP_ENGINE_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::SubGalacticFuelScoop,
    TechnologyId::TransGalacticFuelScoop,
    TechnologyId::TransGalacticMizerScoop
];

pub const ADVANCED_PLANETARY_SCANNER_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Snooper320X,
    TechnologyId::Snooper400X,
    TechnologyId::Snooper620X
];

pub const ADVANCED_SHIP_SCANNER_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::FerretScanner,
    TechnologyId::DolphinScanner,
    TechnologyId::ElephantScanner
];

pub const CLAIM_ADJUSTER_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::RetroBomb,
    TechnologyId::OrbitalAdjuster,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const JACK_OF_ALL_TRADES_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const INTERSTELLAR_TRAVELER_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::StargateAny_300,
    TechnologyId::Stargate100_Any,
    TechnologyId::StargateAny_800,
    TechnologyId::StargateAny_Any,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::AntiMatterGenerator,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const INNER_STRENGTH_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::FuelTransport,
    TechnologyId::SuperFreighter,
    TechnologyId::CrobySharmor,
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::FieldedKelarium,
    TechnologyId::TachyonDetector,
    TechnologyId::MiniGun,
    TechnologyId::Jammer10,
    TechnologyId::Jammer50
];

pub const SPACE_DEMOLITION_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::SuperMineLayer,
    TechnologyId::MiniMineLayer,
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser40,
    TechnologyId::MineDispenser80,
    TechnologyId::MineDispenser130,
    TechnologyId::HeavyDispenser50,
    TechnologyId::HeavyDispenser110,
    TechnologyId::HeavyDispenser200,
    TechnologyId::SpeedTrap20,
    TechnologyId::SpeedTrap30,
    TechnologyId::SpeedTrap50,
    TechnologyId::ColonizationModule,
    TechnologyId::EnergyDampener,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const WAR_MONGER_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::BattleCruiser,
    TechnologyId::Dreadnought,
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::GatlingNeutrinoCannon,
    TechnologyId::Blunderbuss,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const PACKET_PHYSICS_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MassDriver5,
    TechnologyId::MassDriver6,
    TechnologyId::SuperDriver8,
    TechnologyId::SuperDriver9,
    TechnologyId::UltraDriver11,
    TechnologyId::UltraDriver12,
    TechnologyId::UltraDriver13,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const SUPER_STEALTH_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const HYPER_EXPANSION_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::MetaMorph,
    TechnologyId::MiniColonyShip,
    TechnologyId::Viewer50,
    TechnologyId::Viewer90,
    TechnologyId::Scoper150,
    TechnologyId::Scoper220,
    TechnologyId::Scoper280,
    TechnologyId::SDI,
    TechnologyId::MissileBattery,
    TechnologyId::LaserBattery,
    TechnologyId::PlanetaryShield,
    TechnologyId::NeutronShield,
    TechnologyId::MineDispenser50,
    TechnologyId::ColonizationModule,
    TechnologyId::SettlersDelight,
    TechnologyId::FluxCapacitor,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];

pub const ALTERNATE_REALITY_TECHNOLOGY: &'static [TechnologyId] = &[
    TechnologyId::DeathStar,
    TechnologyId::Stargate100_250,
    TechnologyId::Stargate150_600,
    TechnologyId::Stargate300_500,
    TechnologyId::MineDispenser50,
    TechnologyId::OrbitalConstructionModule,
    TechnologyId::SmartBomb,
    TechnologyId::NeutronBomb,
    TechnologyId::EnrichedNeutronBomb,
    TechnologyId::PeerlessBomb,
    TechnologyId::AnnihilatorBomb
];


#[derive(Serialize, Deserialize)]
pub enum ResearchField {
    Energy,
    Weapons,
    Propulsion,
    Construction,
    Electronics,
    Biotechnology
}
impl ResearchField {
    pub fn value(&self) -> usize {
        match *self {
            ResearchField::Energy => 0,
            ResearchField::Weapons => 1,
            ResearchField::Propulsion => 2,
            ResearchField::Construction => 3,
            ResearchField::Electronics => 4,
            ResearchField::Biotechnology => 5,
        }
    }
}
