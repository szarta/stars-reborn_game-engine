/*
 *  Copyright 2018,2019 Brandon Arrendondo
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
const STARGATE_INFINITE_VALUE : u16 = 0;
const DOCK_CAPACITY_INFINITE_VALUE : u16 = 5000;

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
pub enum TechnologySlotType {
    Weapon,
    Electrical,
    Shield,
    Armor,
    Protection,
    OrbitalElect,
    Engine,
    ScannerElectMech,
    GeneralPurpose,
    Mechanical,
    ShieldElectMech,
    WeaponShield,
    MineElectMech,
    Scanner,
    Bomb,
    MiningRobot,
    ArmorScannerElectMech,
    ElectMech,
    MineLayer
}

#[derive(Serialize, Deserialize)]
pub struct TechnologySlot {
    pub slot_type: TechnologySlotType,
    pub amount: u8
}


#[derive(Serialize, Deserialize)]
pub struct TechnologyRequirement {
    pub levels: [u8; 6]
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
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
    pub warp: Option<u8>,
    pub slots: Option<[Option<TechnologySlot>; 16]>,
    pub dock_capacity: Option<u16>,
    pub regen_bonus_percent: Option<u8>
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
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
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gazelle Scanner
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,8,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 0,
            germanium: 5,
            resources: 24
        },
        mass: Some(4),
        basic_range: Some(225),
        penetrating_range: Some(0),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // RNA Scanner
        requirement: TechnologyRequirement {
            levels: [0,0,5,0,0,10]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 1,
            germanium: 2,
            resources: 20
        },
        mass: Some(2),
        basic_range: Some(230),
        penetrating_range: Some(0),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Cheetah Scanner
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,11,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 1,
            germanium: 13,
            resources: 50
        },
        mass: Some(4),
        basic_range: Some(275),
        penetrating_range: Some(0),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Elephant Scanner
        requirement: TechnologyRequirement {
            levels: [6,0,0,0,16,7]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 5,
            germanium: 14,
            resources: 70
        },
        mass: Some(6),
        basic_range: Some(300),
        penetrating_range: Some(200),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Eagle Eye Scanner
        requirement: TechnologyRequirement {
            levels: [6,0,0,0,14,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 2,
            germanium: 21,
            resources: 64
        },
        mass: Some(3),
        basic_range: Some(335),
        penetrating_range: Some(0),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robber Baron Scanner
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,15,10]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 10,
            germanium: 10,
            resources: 90
        },
        mass: Some(20),
        basic_range: Some(220),
        penetrating_range: Some(120),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Peerless Scanner
        requirement: TechnologyRequirement {
            levels: [7,0,0,0,24,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 2,
            germanium: 30,
            resources: 90
        },
        mass: Some(4),
        basic_range: Some(500),
        penetrating_range: Some(0),
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Colonization Module
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 10,
            germanium: 10,
            resources: 10
        },
        mass: Some(32),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Orbital Construction Module
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 15,
            germanium: 15,
            resources: 20
        },
        mass: Some(50),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Cargo Pod
        requirement: TechnologyRequirement {
            levels: [0,0,0,3,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 2,
            resources: 10
        },
        mass: Some(5),
        cargo: Some(50),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Super Cargo Pod
        requirement: TechnologyRequirement {
            levels: [3,0,0,9,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 2,
            resources: 15
        },
        mass: Some(7),
        cargo: Some(100),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, armor: None,
        fuel: None, fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Fuel Tank
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 0,
            resources: 4
        },
        mass: Some(3),
        fuel: Some(250),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Super Fuel Tank
        requirement: TechnologyRequirement {
            levels: [6,0,4,14,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 0,
            resources: 8
        },
        mass: Some(8),
        fuel: Some(500),
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel_per_year: None, battle_speed_modifier: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Manuevering Jet
        requirement: TechnologyRequirement {
            levels: [2,0,3,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 5,
            resources: 10
        },
        mass: Some(5),
        battle_speed_modifier: Some(0.25),
        fuel: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Overthruster
        requirement: TechnologyRequirement {
            levels: [5,0,12,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 8,
            resources: 20
        },
        mass: Some(5),
        battle_speed_modifier: Some(0.5),
        fuel: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_reduction: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Beam Deflector
        requirement: TechnologyRequirement {
            levels: [6,6,0,6,6,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 10,
            resources: 8
        },
        mass: Some(1),
        beam_reduction: Some(10),
        battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        cloaking: None, coverage: None, cargo: None, armor: None,
        fuel_per_year: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Transport Cloaking
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 2,
            resources: 3
        },
        mass: Some(1),
        cloaking: Some(75),
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stealth Cloak
        requirement: TechnologyRequirement {
            levels: [2,0,0,0,5,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 2,
            resources: 5
        },
        mass: Some(2),
        cloaking: Some(35),
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Super Stealth Cloak
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,10,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 8,
            resources: 15
        },
        mass: Some(3),
        cloaking: Some(55),
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Ultra Stealth Cloak
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,12,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 10,
            resources: 25
        },
        mass: Some(5),
        cloaking: Some(85),
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,  jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, initiative: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Battle Computer
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 13,
            resources: 5
        },
        mass: Some(5),
        initiative: Some(1),
        accuracy: Some(20),
        cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Battle Super Computer
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,11,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 25,
            resources: 14
        },
        mass: Some(1),
        initiative: Some(2),
        accuracy: Some(30),
        cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Battle Nexus
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,19,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 30,
            resources: 15
        },
        mass: Some(1),
        initiative: Some(3),
        accuracy: Some(50),
        cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None, jamming: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jammer 10
        requirement: TechnologyRequirement {
            levels: [2,0,0,0,6,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 2,
            resources: 6
        },
        mass: Some(1),
        jamming: Some(10),
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jammer 20
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,10,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 0,
            germanium: 5,
            resources: 20
        },
        mass: Some(1),
        jamming: Some(20),
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jammer 30
        requirement: TechnologyRequirement {
            levels: [8,0,0,0,16,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 0,
            germanium: 6,
            resources: 20
        },
        mass: Some(1),
        jamming: Some(30),
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jammer 50
        requirement: TechnologyRequirement {
            levels: [16,0,0,0,22,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 7,
            resources: 20
        },
        mass: Some(1),
        jamming: Some(50),
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        beam_damage: None, mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Energy Capacitor
        requirement: TechnologyRequirement {
            levels: [7,0,0,0,4,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 8,
            resources: 5
        },
        mass: Some(1),
        beam_damage: Some(10),
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Flux Capacitor
        requirement: TechnologyRequirement {
            levels: [14,0,0,0,8,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 8,
            resources: 5
        },
        mass: Some(1),
        beam_damage: Some(20),
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Energy Dampener
        requirement: TechnologyRequirement {
            levels: [14,0,0,0,8,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 10,
            resources: 50
        },
        mass: Some(2),
        beam_damage: None, jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Tachyon Detector
        requirement: TechnologyRequirement {
            levels: [8,0,0,0,14,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 5,
            germanium: 0,
            resources: 70
        },
        mass: Some(1),
        beam_damage: None, jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        fuel: None, basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        fuel_per_year: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Antimatter Generator
        requirement: TechnologyRequirement {
            levels: [0,12,0,0,0,7]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 3,
            germanium: 3,
            resources: 10
        },
        mass: Some(10),
        fuel: Some(200),
        fuel_per_year: Some(50),
        beam_damage: None, jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_gravity: None, terraforming_radiation: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 3
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(3),
        terraforming_radiation: Some(3),
        terraforming_gravity: Some(3),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 5
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,3]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(5),
        terraforming_radiation: Some(5),
        terraforming_gravity: Some(5),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 7
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,6]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(7),
        terraforming_radiation: Some(7),
        terraforming_gravity: Some(7),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 10
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,9]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(10),
        terraforming_radiation: Some(10),
        terraforming_gravity: Some(10),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 15
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,13]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(15),
        terraforming_radiation: Some(15),
        terraforming_gravity: Some(15),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 20
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,17]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(20),
        terraforming_radiation: Some(20),
        terraforming_gravity: Some(20),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 25
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,22]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(25),
        terraforming_radiation: Some(25),
        terraforming_gravity: Some(25),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Total Terraform 30
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,25]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 70
        },
        terraforming_temperature: Some(30),
        terraforming_radiation: Some(30),
        terraforming_gravity: Some(30),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gravity Terraform 3
        requirement: TechnologyRequirement {
            levels: [0,0,1,0,0,1]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(3),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gravity Terraform 7
        requirement: TechnologyRequirement {
            levels: [0,0,5,0,0,2]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(7),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gravity Terraform 11
        requirement: TechnologyRequirement {
            levels: [0,0,10,0,0,3]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(11),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gravity Terraform 15
        requirement: TechnologyRequirement {
            levels: [0,0,16,0,0,4]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(15),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Temperature Terraform 3
        requirement: TechnologyRequirement {
            levels: [1,0,0,0,0,1]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(3),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Temperature Terraform 7
        requirement: TechnologyRequirement {
            levels: [5,0,0,0,0,2]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(7),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Temperature Terraform 11
        requirement: TechnologyRequirement {
            levels: [10,0,0,0,0,3]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(11),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Temperature Terraform 15
        requirement: TechnologyRequirement {
            levels: [16,0,0,0,0,4]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(15),
        terraforming_radiation: Some(0),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Radiation Terraform 3
        requirement: TechnologyRequirement {
            levels: [0,1,0,0,0,1]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(3),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Radiation Terraform 7
        requirement: TechnologyRequirement {
            levels: [0,5,0,0,0,2]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(7),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Radiation Terraform 11
        requirement: TechnologyRequirement {
            levels: [0,10,0,0,0,3]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(11),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Radiation Terraform 15
        requirement: TechnologyRequirement {
            levels: [0,16,0,0,0,4]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 100
        },
        terraforming_temperature: Some(0),
        terraforming_radiation: Some(15),
        terraforming_gravity: Some(0),
        mass: None, fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None, mining_value: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Midget Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 0,
            germanium: 4,
            resources: 44
        },
        mass: Some(80),
        mining_value: Some(5),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Mini Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,2,1,0]
        },
        cost: TechnologyCost {
            ironium: 29,
            boranium: 0,
            germanium: 7,
            resources: 96
        },
        mass: Some(240),
        mining_value: Some(4),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,4,2,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 0,
            germanium: 7,
            resources: 100
        },
        mass: Some(240),
        mining_value: Some(12),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Maxi Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,7,4,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 0,
            germanium: 7,
            resources: 100
        },
        mass: Some(240),
        mining_value: Some(18),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Super Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,12,6,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 0,
            germanium: 7,
            resources: 100
        },
        mass: Some(240),
        mining_value: Some(27),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Robo Ultra Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,15,8,0]
        },
        cost: TechnologyCost {
            ironium: 14,
            boranium: 0,
            germanium: 4,
            resources: 50
        },
        mass: Some(80),
        mining_value: Some(25),
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None,  cloaking: None,
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Orbital Adjuster
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,6]
        },
        cost: TechnologyCost {
            ironium: 25,
            boranium: 25,
            germanium: 25,
            resources: 50
        },
        mass: Some(80),
        cloaking: Some(25),
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Settlers Delight
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 0,
            germanium: 1,
            resources: 2
        },
        mass: Some(2),
        battle_speed: Some(6),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,0,0,140,275,480,576]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Quick Jump 5
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 1,
            resources: 3
        },
        mass: Some(4),
        battle_speed: Some(5),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,25,100,100,100,180,500,800,900,1080]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Fuel Mizer
        requirement: TechnologyRequirement {
            levels: [0,0,2,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 0,
            germanium: 0,
            resources: 11
        },
        mass: Some(6),
        battle_speed: Some(6),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,35,120,175,235,360,420]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Long Hump 6
        requirement: TechnologyRequirement {
            levels: [0,0,3,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 0,
            germanium: 1,
            resources: 6
        },
        mass: Some(9),
        battle_speed: Some(6),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,20,60,100,100,105,450,750,900,1080]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Daddy Long Legs 7
        requirement: TechnologyRequirement {
            levels: [0,0,5,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 11,
            boranium: 0,
            germanium: 3,
            resources: 12
        },
        mass: Some(13),
        battle_speed: Some(7),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,20,60,70,100,100,110,600,750,900]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Alpha Drive 8
        requirement: TechnologyRequirement {
            levels: [0,0,7,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 16,
            boranium: 0,
            germanium: 3,
            resources: 28
        },
        mass: Some(17),
        battle_speed: Some(8),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,15,50,60,70,100,100,115,700,840]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Trans Galactic Drive
        requirement: TechnologyRequirement {
            levels: [0,0,9,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 20,
            germanium: 9,
            resources: 50
        },
        mass: Some(25),
        battle_speed: Some(9),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,15,35,45,55,70,80,90,100,120]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Interspace 10
        requirement: TechnologyRequirement {
            levels: [0,0,11,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 18,
            boranium: 25,
            germanium: 10,
            resources: 60
        },
        mass: Some(25),
        battle_speed: Some(10),
        warp10_travel: Some(true),
        fuel_table: Some([0,0,10,30,40,50,60,70,80,90,100]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Trans Star 10
        requirement: TechnologyRequirement {
            levels: [0,0,23,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 0,
            germanium: 3,
            resources: 10
        },
        mass: Some(5),
        battle_speed: Some(10),
        warp10_travel: Some(true),
        fuel_table: Some([0,0,5,15,20,25,30,35,40,45,50]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Radiating Hydro Ram Scoop
        requirement: TechnologyRequirement {
            levels: [2,0,6,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 2,
            germanium: 9,
            resources: 8
        },
        mass: Some(10),
        battle_speed: Some(6),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,0,0,165,375,600,720]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Sub Galactic Fuel Scoop
        requirement: TechnologyRequirement {
            levels: [2,0,8,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 4,
            germanium: 7,
            resources: 12
        },
        mass: Some(20),
        battle_speed: Some(7),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,0,85,105,210,380,456]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Trans Galactic Fuel Scoop
        requirement: TechnologyRequirement {
            levels: [3,0,9,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 4,
            germanium: 12,
            resources: 18
        },
        mass: Some(19),
        battle_speed: Some(8),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,0,0,88,100,145,174]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Trans Galactic Super Scoop
        requirement: TechnologyRequirement {
            levels: [4,0,12,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 6,
            boranium: 4,
            germanium: 16,
            resources: 24
        },
        mass: Some(18),
        battle_speed: Some(9),
        warp10_travel: Some(false),
        fuel_table: Some([0,0,0,0,0,0,0,0,65,90,108]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Trans Galactic Mizer Scoop
        requirement: TechnologyRequirement {
            levels: [4,0,16,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 5,
            boranium: 2,
            germanium: 13,
            resources: 20
        },
        mass: Some(11),
        battle_speed: Some(10),
        warp10_travel: Some(true),
        fuel_table: Some([0,0,0,0,0,0,0,0,0,70,84]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Galaxy Scoop
        requirement: TechnologyRequirement {
            levels: [5,0,20,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 2,
            germanium: 9,
            resources: 12
        },
        mass: Some(8),
        battle_speed: Some(10),
        warp10_travel: Some(true),
        fuel_table: Some([0,0,0,0,0,0,0,0,0,0,60]),
        cloaking: None,
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        jamming: None,
        initiative: None, 
        beam_reduction: None, battle_speed_modifier: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Lady Finger Bomb
        requirement: TechnologyRequirement {
            levels: [0,2,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 20,
            germanium: 0,
            resources: 5
        },
        mass: Some(40),
        colonist_kill_percent: Some(0.6),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(2),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Black Cat Bomb
        requirement: TechnologyRequirement {
            levels: [0,5,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 22,
            germanium: 0,
            resources: 7
        },
        mass: Some(45),
        colonist_kill_percent: Some(0.9),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(4),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // M70 Bomb
        requirement: TechnologyRequirement {
            levels: [0,8,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 24,
            germanium: 0,
            resources: 9
        },
        mass: Some(50),
        colonist_kill_percent: Some(1.2),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(6),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // M80 Bomb
        requirement: TechnologyRequirement {
            levels: [0,11,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 25,
            germanium: 0,
            resources: 12
        },
        mass: Some(55),
        colonist_kill_percent: Some(1.7),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(7),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Cherry Bomb
        requirement: TechnologyRequirement {
            levels: [0,14,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 25,
            germanium: 0,
            resources: 11
        },
        mass: Some(52),
        colonist_kill_percent: Some(2.5),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(10),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // LBU17 Bomb
        requirement: TechnologyRequirement {
            levels: [0,5,0,0,8,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 15,
            germanium: 15,
            resources: 7
        },
        mass: Some(30),
        colonist_kill_percent: Some(0.2),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(16),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // LBU32 Bomb
        requirement: TechnologyRequirement {
            levels: [0,10,0,0,10,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 24,
            germanium: 15,
            resources: 10
        },
        mass: Some(35),
        colonist_kill_percent: Some(0.3),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(28),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // LBU74 Bomb
        requirement: TechnologyRequirement {
            levels: [0,15,0,0,12,0]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 33,
            germanium: 12,
            resources: 14
        },
        mass: Some(45),
        colonist_kill_percent: Some(0.4),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(45),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Retro Bomb
        requirement: TechnologyRequirement {
            levels: [0,10,0,0,0,12]
        },
        cost: TechnologyCost {
            ironium: 15,
            boranium: 15,
            germanium: 10,
            resources: 50
        },
        mass: Some(45),
        colonist_kill_percent: Some(0.0),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(false),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Smart Bomb
        requirement: TechnologyRequirement {
            levels: [0,5,0,0,0,7]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 22,
            germanium: 0,
            resources: 27
        },
        mass: Some(50),
        colonist_kill_percent: Some(1.3),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(true),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Neutron Bomb
        requirement: TechnologyRequirement {
            levels: [0,10,0,0,0,10]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 30,
            germanium: 0,
            resources: 30
        },
        mass: Some(57),
        colonist_kill_percent: Some(2.2),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(true),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Enriched Neutron Bomb
        requirement: TechnologyRequirement {
            levels: [0,15,0,0,0,12]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 36,
            germanium: 0,
            resources: 25
        },
        mass: Some(64),
        colonist_kill_percent: Some(3.5),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(true),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Peerless Bomb
        requirement: TechnologyRequirement {
            levels: [0,22,0,0,0,15]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 33,
            germanium: 0,
            resources: 32
        },
        mass: Some(55),
        colonist_kill_percent: Some(5.0),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(true),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Annihilator Bomb
        requirement: TechnologyRequirement {
            levels: [0,26,0,0,0,17]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 30,
            germanium: 0,
            resources: 28
        },
        mass: Some(50),
        colonist_kill_percent: Some(7.0),
        min_colonists_killed: Some(0),
        buildings_destroyed: Some(0),
        is_smart: Some(true),
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate 100 250
        requirement: TechnologyRequirement {
            levels: [0,0,5,5,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 200
        },
        safe_mass: Some(100),
        safe_distance: Some(250),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate Any 300
        requirement: TechnologyRequirement {
            levels: [0,0,6,10,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 250
        },
        safe_mass: Some(STARGATE_INFINITE_VALUE),
        safe_distance: Some(300),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate 150 600
        requirement: TechnologyRequirement {
            levels: [0,0,11,7,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 500
        },
        safe_mass: Some(150),
        safe_distance: Some(600),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate 300 500
        requirement: TechnologyRequirement {
            levels: [0,0,9,13,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 600
        },
        safe_mass: Some(300),
        safe_distance: Some(500),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate 100 Any
        requirement: TechnologyRequirement {
            levels: [0,0,16,12,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 700
        },
        safe_mass: Some(100),
        safe_distance: Some(STARGATE_INFINITE_VALUE),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate Any 800
        requirement: TechnologyRequirement {
            levels: [0,0,12,18,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 700
        },
        safe_mass: Some(STARGATE_INFINITE_VALUE),
        safe_distance: Some(800),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Stargate Any Any
        requirement: TechnologyRequirement {
            levels: [0,0,19,24,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 20,
            germanium: 20,
            resources: 800
        },
        safe_mass: Some(STARGATE_INFINITE_VALUE),
        safe_distance: Some(STARGATE_INFINITE_VALUE),
        mass: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mass Driver 5
        requirement: TechnologyRequirement {
            levels: [4,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 70
        },
        warp: Some(5),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mass Driver 6
        requirement: TechnologyRequirement {
            levels: [7,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 144
        },
        warp: Some(6),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mass Driver 7
        requirement: TechnologyRequirement {
            levels: [9,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 100,
            boranium: 100,
            germanium: 100,
            resources: 512
        },
        warp: Some(7),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Super Driver 8
        requirement: TechnologyRequirement {
            levels: [11,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 256
        },
        warp: Some(8),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Super Driver 9
        requirement: TechnologyRequirement {
            levels: [13,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 324
        },
        warp: Some(9),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Ultra Driver 10
        requirement: TechnologyRequirement {
            levels: [15,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 100,
            boranium: 100,
            germanium: 100,
            resources: 968
        },
        warp: Some(10),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Ultra Driver 11
        requirement: TechnologyRequirement {
            levels: [17,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 484
        },
        warp: Some(11),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Ultra Driver 12
        requirement: TechnologyRequirement {
            levels: [20,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 576
        },
        warp: Some(12),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Ultra Driver 13
        requirement: TechnologyRequirement {
            levels: [24,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 24,
            boranium: 20,
            germanium: 20,
            resources: 676
        },
        warp: Some(13),
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
        slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Orbital Fort
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 0,
            germanium: 17,
            resources: 40
        },
        armor: Some(100),
        initiative: Some(10),
        dock_capacity: Some(0),
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 12
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 12
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 12
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 12
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, regen_bonus_percent: None
    },
    Technology { // Space Dock
        requirement: TechnologyRequirement {
            levels: [0,0,0,4,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 5,
            germanium: 25,
            resources: 100
        },
        armor: Some(250),
        initiative: Some(12),
        dock_capacity: Some(200),
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 24
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 24
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, regen_bonus_percent: None
    },
    Technology { // Space Station
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 120,
            boranium: 80,
            germanium: 250,
            resources: 600
        },
        armor: Some(500),
        initiative: Some(14),
        dock_capacity: Some(DOCK_CAPACITY_INFINITE_VALUE),
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
               Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
               Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, regen_bonus_percent: None
    },
    Technology { // Ultra Station
        requirement: TechnologyRequirement {
            levels: [0,0,0,17,0,0]
        },
        cost: TechnologyCost {
            ironium: 120,
            boranium: 80,
            germanium: 300,
            resources: 600
        },
        armor: Some(1000),
        initiative: Some(16),
        dock_capacity: Some(DOCK_CAPACITY_INFINITE_VALUE),
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 16
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 20
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 20
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 20
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 20
                }),
              Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
               Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
        ]),
        warp: None,
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, regen_bonus_percent: None
    },
    Technology { // Death Star
        requirement: TechnologyRequirement {
            levels: [0,0,0,17,0,0]
        },
        cost: TechnologyCost {
            ironium: 120,
            boranium: 80,
            germanium: 350,
            resources: 750
        },
        armor: Some(1500),
        initiative: Some(16),
        dock_capacity: Some(DOCK_CAPACITY_INFINITE_VALUE),
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 32
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 32
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 32
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 32
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 30
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 30
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 20
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 20
                }),
              Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
               Some(TechnologySlot {
                    slot_type: TechnologySlotType::OrbitalElect,
                    amount: 1
                }),
        ]),
        warp: None,
        safe_mass: None, safe_distance: None, mass: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None,
        power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, regen_bonus_percent: None
    },
    Technology { // Small Freighter
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 0,
            germanium: 17,
            resources: 20
        },
        armor: Some(25),
        mass: Some(25),
        initiative: Some(0),
        cargo: Some(70),
        fuel: Some(130),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Medium Freighter
        requirement: TechnologyRequirement {
            levels: [0,0,0,3,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 0,
            germanium: 19,
            resources: 40
        },
        armor: Some(50),
        mass: Some(60),
        initiative: Some(0),
        cargo: Some(210),
        fuel: Some(450),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Large Freighter
        requirement: TechnologyRequirement {
            levels: [0,0,0,8,0,0]
        },
        cost: TechnologyCost {
            ironium: 35,
            boranium: 0,
            germanium: 21,
            resources: 100
        },
        armor: Some(150),
        mass: Some(125),
        initiative: Some(0),
        cargo: Some(1200),
        fuel: Some(2600),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Super Freighter
        requirement: TechnologyRequirement {
            levels: [0,0,0,13,0,0]
        },
        cost: TechnologyCost {
            ironium: 45,
            boranium: 0,
            germanium: 21,
            resources: 125
        },
        armor: Some(400),
        mass: Some(175),
        initiative: Some(0),
        cargo: Some(3000),
        fuel: Some(8000),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 5
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Scout
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 2,
            germanium: 4,
            resources: 10
        },
        armor: Some(20),
        mass: Some(8),
        initiative: Some(1),
        cargo: None,
        fuel: Some(50),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Scanner,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Frigate
        requirement: TechnologyRequirement {
            levels: [0,0,0,6,0,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 2,
            germanium: 4,
            resources: 12
        },
        armor: Some(45),
        mass: Some(8),
        initiative: Some(4),
        cargo: None,
        fuel: Some(125),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Scanner,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Destroyer
        requirement: TechnologyRequirement {
            levels: [0,0,0,3,0,0]
        },
        cost: TechnologyCost {
            ironium: 15,
            boranium: 3,
            germanium: 5,
            resources: 35
        },
        armor: Some(200),
        mass: Some(30),
        initiative: Some(3),
        cargo: None,
        fuel: Some(280),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Armor,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 1
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::Mechanical,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Cruiser
        requirement: TechnologyRequirement {
            levels: [0,0,0,9,0,0]
        },
        cost: TechnologyCost {
            ironium: 40,
            boranium: 5,
            germanium: 8,
            resources: 85
        },
        armor: Some(700),
        mass: Some(90),
        initiative: Some(5),
        cargo: None,
        fuel: Some(600),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ShieldElectMech,
                    amount: 1
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::ShieldElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Battle Cruiser
        requirement: TechnologyRequirement {
            levels: [0,0,0,10,0,0]
        },
        cost: TechnologyCost {
            ironium: 55,
            boranium: 8,
            germanium: 12,
            resources: 120
        },
        armor: Some(1000),
        mass: Some(120),
        initiative: Some(5),
        cargo: None,
        fuel: Some(1400),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ShieldElectMech,
                    amount: 2
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::ShieldElectMech,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Battleship
        requirement: TechnologyRequirement {
            levels: [0,0,0,13,0,0]
        },
        cost: TechnologyCost {
            ironium: 120,
            boranium: 25,
            germanium: 20,
            resources: 225
        },
        armor: Some(2000),
        mass: Some(222),
        initiative: Some(10),
        cargo: None,
        fuel: Some(2800),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 6
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 6
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Armor,
                    amount: 6
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 8
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Dreadnought
        requirement: TechnologyRequirement {
            levels: [0,0,0,16,0,0]
        },
        cost: TechnologyCost {
            ironium: 140,
            boranium: 30,
            germanium: 25,
            resources: 275
        },
        armor: Some(4500),
        mass: Some(250),
        initiative: Some(10),
        cargo: None,
        fuel: Some(4500),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 5
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 6
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 6
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Weapon,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Armor,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::WeaponShield,
                    amount: 5
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::WeaponShield,
                    amount: 5
                }),
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Privateer
        requirement: TechnologyRequirement {
            levels: [0,0,0,4,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 3,
            germanium: 2,
            resources: 50
        },
        armor: Some(150),
        mass: Some(65),
        initiative: Some(3),
        cargo: Some(250),
        fuel: Some(650),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Rogue
        requirement: TechnologyRequirement {
            levels: [0,0,0,8,0,0]
        },
        cost: TechnologyCost {
            ironium: 80,
            boranium: 5,
            germanium: 5,
            resources: 60
        },
        armor: Some(450),
        mass: Some(75),
        initiative: Some(4),
        cargo: Some(500),
        fuel: Some(2250),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 1
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineElectMech,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineElectMech,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Scanner,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Galleon
        requirement: TechnologyRequirement {
            levels: [0,0,0,11,0,0]
        },
        cost: TechnologyCost {
            ironium: 70,
            boranium: 5,
            germanium: 5,
            resources: 105
        },
        armor: Some(900),
        mass: Some(125),
        initiative: Some(4),
        cargo: Some(1000),
        fuel: Some(2500),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                 Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineElectMech,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ElectMech,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Scanner,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Mini Colony Ship
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 2,
            boranium: 0,
            germanium: 2,
            resources: 3
        },
        armor: Some(10),
        mass: Some(8),
        initiative: Some(0),
        cargo: Some(10),
        fuel: Some(150),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Mechanical,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Colony Ship
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 15,
            resources: 20
        },
        armor: Some(20),
        mass: Some(20),
        initiative: Some(0),
        cargo: Some(25),
        fuel: Some(200),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Mechanical,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Mini Bomber
        requirement: TechnologyRequirement {
            levels: [0,0,0,1,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 5,
            germanium: 10,
            resources: 35
        },
        armor: Some(50),
        mass: Some(28),
        initiative: Some(0),
        cargo: None,
        fuel: Some(120),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // B17 Bomber
        requirement: TechnologyRequirement {
            levels: [0,0,0,6,0,0]
        },
        cost: TechnologyCost {
            ironium: 55,
            boranium: 10,
            germanium: 10,
            resources: 150
        },
        armor: Some(175),
        mass: Some(69),
        initiative: Some(0),
        cargo: None,
        fuel: Some(400),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Stealth Bomber
        requirement: TechnologyRequirement {
            levels: [0,0,0,8,0,0]
        },
        cost: TechnologyCost {
            ironium: 55,
            boranium: 10,
            germanium: 15,
            resources: 175
        },
        armor: Some(225),
        mass: Some(70),
        initiative: Some(0),
        cargo: None,
        fuel: Some(750),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Electrical,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // B52 Bomber
        requirement: TechnologyRequirement {
            levels: [0,0,0,15,0,0]
        },
        cost: TechnologyCost {
            ironium: 90,
            boranium: 15,
            germanium: 10,
            resources: 280
        },
        armor: Some(450),
        mass: Some(110),
        initiative: Some(0),
        cargo: None,
        fuel: Some(750),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Bomb,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Midget Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 3,
            resources: 20
        },
        armor: Some(100),
        mass: Some(10),
        initiative: Some(0),
        cargo: None,
        fuel: Some(210),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Mini Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,2,0,0]
        },
        cost: TechnologyCost {
            ironium: 25,
            boranium: 0,
            germanium: 6,
            resources: 50
        },
        armor: Some(130),
        mass: Some(80),
        initiative: Some(0),
        cargo: None,
        fuel: Some(210),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,6,0,0]
        },
        cost: TechnologyCost {
            ironium: 32,
            boranium: 0,
            germanium: 6,
            resources: 110
        },
        armor: Some(475),
        mass: Some(110),
        initiative: Some(0),
        cargo: None,
        fuel: Some(500),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ArmorScannerElectMech,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Maxi Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,11,0,0]
        },
        cost: TechnologyCost {
            ironium: 32,
            boranium: 0,
            germanium: 6,
            resources: 140
        },
        armor: Some(1400),
        mass: Some(110),
        initiative: Some(0),
        cargo: None,
        fuel: Some(850),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ArmorScannerElectMech,
                    amount: 2
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Ultra Miner
        requirement: TechnologyRequirement {
            levels: [0,0,0,14,0,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 0,
            germanium: 6,
            resources: 130
        },
        armor: Some(1500),
        mass: Some(100),
        initiative: Some(0),
        cargo: None,
        fuel: Some(1300),
        dock_capacity: None, regen_bonus_percent: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 4
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MiningRobot,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ArmorScannerElectMech,
                    amount: 3
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Fuel Transport
        requirement: TechnologyRequirement {
            levels: [0,0,0,4,0,0]
        },
        cost: TechnologyCost {
            ironium: 10,
            boranium: 0,
            germanium: 5,
            resources: 50
        },
        armor: Some(5),
        mass: Some(12),
        initiative: Some(0),
        cargo: None,
        fuel: Some(750),
        fuel_per_year: Some(200),
        regen_bonus_percent: Some(5),
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 3
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Super Fuel Transport
        requirement: TechnologyRequirement {
            levels: [0,0,0,7,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 0,
            germanium: 8,
            resources: 70
        },
        armor: Some(12),
        mass: Some(111),
        initiative: Some(0),
        cargo: None,
        fuel: Some(2250),
        fuel_per_year: Some(200),
        regen_bonus_percent: Some(10),
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Shield,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Scanner,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Mini Mine Layer
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 2,
            germanium: 5,
            resources: 20
        },
        armor: Some(60),
        mass: Some(10),
        initiative: Some(0),
        cargo: None,
        fuel: Some(400),
        fuel_per_year: None,
        regen_bonus_percent: None,
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineLayer,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineLayer,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Super Mine Layer
        requirement: TechnologyRequirement {
            levels: [0,0,0,15,0,0]
        },
        cost: TechnologyCost {
            ironium: 20,
            boranium: 3,
            germanium: 9,
            resources: 30
        },
        armor: Some(1200),
        mass: Some(30),
        initiative: Some(0),
        cargo: None,
        fuel: Some(2200),
        fuel_per_year: None,
        regen_bonus_percent: None,
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineLayer,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineLayer,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::ScannerElectMech,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Protection,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::MineElectMech,
                    amount: 3
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Nubian
        requirement: TechnologyRequirement {
            levels: [0,0,0,26,0,0]
        },
        cost: TechnologyCost {
            ironium: 75,
            boranium: 12,
            germanium: 12,
            resources: 150
        },
        armor: Some(5000),
        mass: Some(100),
        initiative: Some(2),
        cargo: None,
        fuel: Some(5000),
        fuel_per_year: None,
        regen_bonus_percent: None,
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Metamorph
        requirement: TechnologyRequirement {
            levels: [0,0,0,8,0,0]
        },
        cost: TechnologyCost {
            ironium: 50,
            boranium: 12,
            germanium: 12,
            resources: 120
        },
        armor: Some(500),
        mass: Some(85),
        initiative: Some(2),
        cargo: Some(300),
        fuel: Some(700),
        fuel_per_year: None,
        regen_bonus_percent: None,
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 3
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 8
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
    },
    Technology { // Laser
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 6,
            germanium: 0,
            resources: 5
        },
        mass: Some(1),
        power: Some(10),
        range: Some(1),
        initiative: Some(9),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Xray Laser
        requirement: TechnologyRequirement {
            levels: [0,3,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 6,
            germanium: 0,
            resources: 6
        },
        mass: Some(1),
        power: Some(16),
        range: Some(1),
        initiative: Some(9),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Minigun
        requirement: TechnologyRequirement {
            levels: [0,5,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 16,
            germanium: 0,
            resources: 10
        },
        mass: Some(3),
        power: Some(13),
        range: Some(2),
        initiative: Some(12),
        is_spread: Some(true),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Yakimora Light Phaser
        requirement: TechnologyRequirement {
            levels: [0,6,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 8,
            germanium: 0,
            resources: 7
        },
        mass: Some(1),
        power: Some(26),
        range: Some(1),
        initiative: Some(9),
        is_spread: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Blackjack
        requirement: TechnologyRequirement {
            levels: [0,7,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 16,
            germanium: 0,
            resources: 7
        },
        mass: Some(10),
        power: Some(90),
        range: Some(0),
        initiative: Some(10),
        is_spread: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Phaser Bazooka
        requirement: TechnologyRequirement {
            levels: [0,8,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 8,
            germanium: 0,
            resources: 11
        },
        mass: Some(2),
        power: Some(26),
        range: Some(2),
        initiative: Some(7),
        is_spread: None,
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Pulsed Sapper
        requirement: TechnologyRequirement {
            levels: [5,9,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 4,
            resources: 12
        },
        mass: Some(1),
        power: Some(82),
        range: Some(3),
        initiative: Some(14),
        is_spread: None,
        hits_shields_only: Some(true),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Colloidal Phaser
        requirement: TechnologyRequirement {
            levels: [0,10,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 14,
            germanium: 0,
            resources: 18
        },
        mass: Some(2),
        power: Some(26),
        range: Some(3),
        initiative: Some(5),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gatling Gun
        requirement: TechnologyRequirement {
            levels: [0,11,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 20,
            germanium: 0,
            resources: 13
        },
        mass: Some(3),
        power: Some(31),
        range: Some(2),
        initiative: Some(12),
        is_spread: Some(true), hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mini Blaster
        requirement: TechnologyRequirement {
            levels: [0,12,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 10,
            germanium: 0,
            resources: 9
        },
        mass: Some(1),
        power: Some(66),
        range: Some(1),
        initiative: Some(9),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Bludgeon
        requirement: TechnologyRequirement {
            levels: [0,13,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 22,
            germanium: 0,
            resources: 9
        },
        mass: Some(10),
        power: Some(231),
        range: Some(0),
        initiative: Some(10),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mark IV Blaster
        requirement: TechnologyRequirement {
            levels: [0,14,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 12,
            germanium: 0,
            resources: 15
        },
        mass: Some(2),
        power: Some(66),
        range: Some(2),
        initiative: Some(7),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Phased Sapper
        requirement: TechnologyRequirement {
            levels: [8,15,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 6,
            resources: 16
        },
        mass: Some(1),
        power: Some(211),
        range: Some(3),
        initiative: Some(14),
        is_spread: None, hits_shields_only: Some(true), colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Heavy Blaster
        requirement: TechnologyRequirement {
            levels: [0,16,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 20,
            germanium: 0,
            resources: 25
        },
        mass: Some(2),
        power: Some(66),
        range: Some(3),
        initiative: Some(5),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Gatling Neutrino Cannon
        requirement: TechnologyRequirement {
            levels: [0,17,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 28,
            germanium: 0,
            resources: 17
        },
        mass: Some(3),
        power: Some(80),
        range: Some(2),
        initiative: Some(13),
        is_spread: Some(true), hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Myopic Disruptor
        requirement: TechnologyRequirement {
            levels: [0,18,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 14,
            germanium: 0,
            resources: 12
        },
        mass: Some(1),
        power: Some(169),
        range: Some(1),
        initiative: Some(9),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Blunderbuss
        requirement: TechnologyRequirement {
            levels: [0,19,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 30,
            germanium: 0,
            resources: 13
        },
        mass: Some(10),
        power: Some(592),
        range: Some(0),
        initiative: Some(11),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Disruptor
        requirement: TechnologyRequirement {
            levels: [0,20,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 16,
            germanium: 0,
            resources: 20
        },
        mass: Some(2),
        power: Some(169),
        range: Some(2),
        initiative: Some(8),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Syncro Sapper
        requirement: TechnologyRequirement {
            levels: [11,21,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 8,
            resources: 21
        },
        mass: Some(1),
        power: Some(541),
        range: Some(3),
        initiative: Some(14),
        is_spread: None, hits_shields_only: Some(true), colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mega Disruptor
        requirement: TechnologyRequirement {
            levels: [0,22,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 30,
            germanium: 0,
            resources: 33
        },
        mass: Some(2),
        power: Some(169),
        range: Some(3),
        initiative: Some(6),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Big Mutha Cannon
        requirement: TechnologyRequirement {
            levels: [0,23,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 36,
            germanium: 0,
            resources: 23
        },
        mass: Some(3),
        power: Some(204),
        range: Some(2),
        initiative: Some(13),
        is_spread: Some(true), hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Streaming Pulverizer
        requirement: TechnologyRequirement {
            levels: [0,24,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 20,
            germanium: 0,
            resources: 16
        },
        mass: Some(1),
        power: Some(433),
        range: Some(1),
        initiative: Some(9),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Anti-Matter Pulverizer
        requirement: TechnologyRequirement {
            levels: [0,26,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 20,
            germanium: 0,
            resources: 27
        },
        mass: Some(2),
        power: Some(433),
        range: Some(2),
        initiative: Some(8),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Alpha Torpedo
        requirement: TechnologyRequirement {
            levels: [0,0,0,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 8,
            boranium: 3,
            germanium: 3,
            resources: 4
        },
        mass: Some(25),
        power: Some(5),
        range: Some(4),
        initiative: Some(0),
        accuracy: Some(35),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Beta Torpedo
        requirement: TechnologyRequirement {
            levels: [0,5,1,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 18,
            boranium: 6,
            germanium: 4,
            resources: 6
        },
        mass: Some(25),
        power: Some(12),
        range: Some(4),
        initiative: Some(1),
        accuracy: Some(45),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Delta Torpedo
        requirement: TechnologyRequirement {
            levels: [0,10,2,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 22,
            boranium: 8,
            germanium: 5,
            resources: 8
        },
        mass: Some(25),
        power: Some(26),
        range: Some(4),
        initiative: Some(1),
        accuracy: Some(60),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Epsilon Torpedo
        requirement: TechnologyRequirement {
            levels: [0,14,3,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 10,
            germanium: 6,
            resources: 10 
        },
        mass: Some(25),
        power: Some(48),
        range: Some(5),
        initiative: Some(2),
        accuracy: Some(65),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Rho Torpedo
        requirement: TechnologyRequirement {
            levels: [0,18,4,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 34,
            boranium: 12,
            germanium: 8,
            resources: 12 
        },
        mass: Some(25),
        power: Some(90),
        range: Some(5),
        initiative: Some(2),
        accuracy: Some(75),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Upsilon Torpedo
        requirement: TechnologyRequirement {
            levels: [0,22,5,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 40,
            boranium: 14,
            germanium: 9,
            resources: 15 
        },
        mass: Some(25),
        power: Some(169),
        range: Some(5),
        initiative: Some(3),
        accuracy: Some(75),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Omega Torpedo
        requirement: TechnologyRequirement {
            levels: [0,26,6,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 52,
            boranium: 18,
            germanium: 12,
            resources: 18 
        },
        mass: Some(25),
        power: Some(316),
        range: Some(5),
        initiative: Some(4),
        accuracy: Some(80),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jihad Missile
        requirement: TechnologyRequirement {
            levels: [0,12,6,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 37,
            boranium: 13,
            germanium: 9,
            resources: 13 
        },
        mass: Some(35),
        power: Some(85),
        range: Some(5),
        initiative: Some(0),
        accuracy: Some(20),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Juggernaut Missile
        requirement: TechnologyRequirement {
            levels: [0,16,8,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 48,
            boranium: 16,
            germanium: 11,
            resources: 16 
        },
        mass: Some(35),
        power: Some(150),
        range: Some(5),
        initiative: Some(1),
        accuracy: Some(20),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Doomsday Missile
        requirement: TechnologyRequirement {
            levels: [0,20,10,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 60,
            boranium: 20,
            germanium: 13,
            resources: 20 
        },
        mass: Some(35),
        power: Some(280),
        range: Some(6),
        initiative: Some(2),
        accuracy: Some(25),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Armageddon Missile
        requirement: TechnologyRequirement {
            levels: [0,24,10,0,0,0]
        },
        cost: TechnologyCost {
            ironium: 67,
            boranium: 23,
            germanium: 16,
            resources: 24 
        },
        mass: Some(35),
        power: Some(525),
        range: Some(6),
        initiative: Some(3),
        accuracy: Some(30),
        is_spread: None, hits_shields_only: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },

    //                                                                               
    // MT part values courtesy of:                                                   
    //   http://wiki.starsautohost.org/wiki/Mystery_Trader                           
    // 

    Technology { // Hushaboom
        requirement: TechnologyRequirement {
            levels: [0,12,0,0,12,12]
        },
        cost: TechnologyCost {
            ironium: 1,
            boranium: 2,
            germanium: 0,
            resources: 2 
        },
        colonist_kill_percent: Some(3.0),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(2),
        mass: Some(5),
        power: None,
        range: None,
        initiative: None,
        accuracy: None,
        is_spread: None, hits_shields_only: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Enigma Pulsar
        requirement: TechnologyRequirement {
            levels: [7,0,13,5,9,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 15,
            germanium: 11,
            resources: 40
        },
        mass: Some(20),
        battle_speed: Some(10),
        warp10_travel: Some(true),
        fuel_table: Some([0,0,0,0,0,0,0,0,0,0,60]),
        cloaking: Some(10),
        battle_speed_modifier: Some(0.25),
        mining_value: None,
        terraforming_temperature: None,
        terraforming_radiation: None,
        terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None, jamming: None,
        initiative: None,  beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, cargo: None, armor: None,
        is_smart: None, colonist_kill_percent: None,
        min_colonists_killed: None, buildings_destroyed: None, power: None,
        range: None, is_spread: None,
        hits_shields_only: None, accuracy: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Mega Poly Shell
        requirement: TechnologyRequirement {
            levels: [14,0,0,14,14,6]
        },
        cost: TechnologyCost {
            ironium: 14,
            boranium: 5,
            germanium: 5,
            resources: 52 
        },
        cloaking: Some(20),
        jamming: Some(20),
        shield_value: Some(100),
        basic_range: Some(80),
        penetrating_range: Some(40),
        mass: Some(20),
        armor: Some(400),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        power: None,
        range: None,
        initiative: None,
        accuracy: None,
        is_spread: None, hits_shields_only: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Langston Shell
        requirement: TechnologyRequirement {
            levels: [12,0,9,0,9,0]
        },
        cost: TechnologyCost {
            ironium: 6,
            boranium: 1,
            germanium: 4,
            resources: 12 
        },
        cloaking: Some(10),
        jamming: Some(5),
        shield_value: Some(125),
        basic_range: Some(50),
        penetrating_range: Some(25),
        mass: Some(10),
        armor: Some(65),
        colonist_kill_percent: None,
        min_colonists_killed: None,
        buildings_destroyed: None,
        power: None,
        range: None,
        initiative: None,
        accuracy: None,
        is_spread: None, hits_shields_only: None,
        is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Multi Function Pod
        requirement: TechnologyRequirement {
            levels: [11,0,11,0,11,0]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 0,
            germanium: 4,
            resources: 13 
        },
        cloaking: Some(30),
        jamming: Some(10),
        mass: Some(2),
        battle_speed_modifier: Some(0.25),
        shield_value: None, basic_range: None, penetrating_range: None,
        armor: None, colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, power: None, range: None, initiative: None,
        accuracy: None, is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Anti Matter Torpedo
        requirement: TechnologyRequirement {
            levels: [0,11,12,0,0,21]
        },
        cost: TechnologyCost {
            ironium: 3,
            boranium: 8,
            germanium: 1,
            resources: 50 
        },
        mass: Some(8),
        power: Some(60),
        range: Some(6),
        initiative: Some(0),
        accuracy: Some(85),
        cloaking: None,
        jamming: None,
        battle_speed_modifier: None,
        shield_value: None, basic_range: None, penetrating_range: None,
        armor: None, colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, 
        is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Jump Gate
        requirement: TechnologyRequirement {
            levels: [16,0,20,20,16,0]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 38,
            resources: 30 
        },
        mass: Some(10),
        power: None, range: None, initiative: None, accuracy: None,
        cloaking: None, jamming: None, battle_speed_modifier: None,
        shield_value: None, basic_range: None, penetrating_range: None,
        armor: None, colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, 
        is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Genesis Device
        requirement: TechnologyRequirement {
            levels: [20,10,10,20,10,20]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 0,
            germanium: 0,
            resources: 5000
        },
        mass: None,
        power: None, range: None, initiative: None, accuracy: None,
        cloaking: None, jamming: None, battle_speed_modifier: None,
        shield_value: None, basic_range: None, penetrating_range: None,
        armor: None, colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, 
        is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Multi Contained Munition
        requirement: TechnologyRequirement {
            levels: [21,21,0,0,16,12]
        },
        cost: TechnologyCost {
            ironium: 0,
            boranium: 6,
            germanium: 0,
            resources: 5
        },
        mass: Some(8),
        power: Some(140),
        range: Some(3),
        initiative: Some(6),
        accuracy: Some(10),
        cloaking: Some(10), 
        basic_range: Some(150),
        penetrating_range: Some(75),
        mines_per_year: Some(40),
        mine_type: Some(MineType::Normal),
        colonist_kill_percent: Some(2.0),
        min_colonists_killed: Some(300),
        buildings_destroyed: Some(5),
        jamming: None, battle_speed_modifier: None, shield_value: None,
        armor: None, is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None, coverage: None, cargo: None, safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Alien Miner
        requirement: TechnologyRequirement {
            levels: [5,0,0,10,5,5]
        },
        cost: TechnologyCost {
            ironium: 4,
            boranium: 0,
            germanium: 1,
            resources: 10
        },
        mass: Some(20),
        mining_value: Some(10),
        cloaking: Some(30),
        jamming: Some(30),
        battle_speed_modifier: Some(1.0 / 8.0),
        power: None, range: None, initiative: None, accuracy: None,
        shield_value: None, basic_range: None, penetrating_range: None,
        armor: None, colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, 
        is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None, cargo: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Multi Cargo Pod
        requirement: TechnologyRequirement {
            levels: [5,0,0,11,5,0]
        },
        cost: TechnologyCost {
            ironium: 12,
            boranium: 0,
            germanium: 3,
            resources: 25
        },
        mass: Some(9),
        armor: Some(50),
        mining_value: None,
        cloaking: Some(10),
        cargo: Some(250),
        jamming: None,
        battle_speed_modifier: None,
        power: None, range: None, initiative: None, accuracy: None,
        shield_value: None, basic_range: None, penetrating_range: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, 
        is_spread: None, hits_shields_only: None,
        is_smart: None, battle_speed: None, warp10_travel: None, fuel_table: None,
        terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        fuel: None, fuel_per_year: None, beam_damage: None,
        beam_reduction: None,
        mines_per_year: None, mine_type: None,
        coverage: None,
        safe_mass: None,
        safe_distance: None, warp: None, slots: None, dock_capacity: None, regen_bonus_percent: None
    },
    Technology { // Minimorph
        requirement: TechnologyRequirement {
            levels: [0,0,0,8,0,0]
        },
        cost: TechnologyCost {
            ironium: 30,
            boranium: 8,
            germanium: 8,
            resources: 100
        },
        armor: Some(25),
        mass: Some(70),
        initiative: Some(2),
        cargo: Some(150),
        fuel: Some(400),
        fuel_per_year: None,
        regen_bonus_percent: None,
        dock_capacity: None,
        slots: Some([
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::Engine,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 2
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 1
                }),
                Some(TechnologySlot {
                    slot_type: TechnologySlotType::GeneralPurpose,
                    amount: 3
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
        ]),
        warp: None,
        safe_mass: None, safe_distance: None,
        colonist_kill_percent: None, min_colonists_killed: None,
        buildings_destroyed: None, is_smart: None,
        battle_speed: None, warp10_travel: None, fuel_table: None,
        cloaking: None, battle_speed_modifier: None,
        mining_value: None, terraforming_temperature: None,
        terraforming_radiation: None, terraforming_gravity: None,
        beam_damage: None, jamming: None,
         beam_reduction: None,
        basic_range: None, penetrating_range: None,
        mines_per_year: None, mine_type: None, shield_value: None,
        coverage: None, power: None, range: None, is_spread: None,
        hits_shields_only: None, accuracy: None,
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
