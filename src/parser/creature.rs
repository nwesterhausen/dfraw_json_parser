use super::raws::names;
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActiveTimeKind {
    Diurnal,     // day
    Nocturnal,   // night
    Crepuscular, // twilight
    All,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AggressionKind {
    LargePredator,
    AmbushPredator,
    Benign,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BreathesKind {
    Amphibious, // breathes in and out of water
    Aquatic,    // breathes in water but air-drown on land
    Air,        // normal?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DietType {
    Carnivore,
    BoneCarnivore, // eats bones, implies carnivore
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodySize {
    years: u32,
    days: u32,
    cm3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    identifier: String,
    parent_raw: String,
    #[serde(rename = "objectId")]
    object_id: String,
    pub names: Vec<names::Name>,
    pub child_names: Vec<names::SingPlurName>,
    pub lays_eggs: bool,
    pub egg_size: u32,
    pub clutch_size: [u32; 2],
    pub description: String,
    pub max_age: [u32; 2],
    pub based_on: String,

    pub adopts_owner: bool,
    pub alcohol_dep: bool,
    pub active_time: ActiveTimeKind,
    pub tiles: [char; 2],               // tile and alt-tile
    pub aggression_kind: AggressionKind, // ambush_predator, large_predator, benign etc
    pub breathes_kind: BreathesKind,
    pub arena_restricted: bool,    // true if not allowed spawning in arena
    pub artificial_hivable: bool,  // true if can be kept in hives by beekeeper
    pub peace_with_wildlife: bool, // true prevents creature from attacking/frightening creatures that natural=true
    pub baby_until: u32,           // age at which becomes a child, def: 0
    pub baby_names: Vec<names::SingPlurName>, // Also should include GENERAL_BABY_NAME
    pub beach_frequency: u32,      // freq of getting stranded on shores, e.g. orcas, sperm whales
    pub biome: Vec<String>,
    pub bloodsucker: bool,
    pub body_size: Vec<BodySize>,
    // Should have sub structs for body, body-parts
    pub diet: DietType,
    pub building_destroyer: u8, // 0 not, 1 targets doors, hatches, furn, 2 targets all non-constructions
    pub can_learn: bool,
    pub can_speak: bool,
    pub cannot_climb: bool,
    pub cannot_jump: bool,
    pub cannot_undead: bool,
    pub can_open_doors: bool,
    // Should have sub structs for caste
    pub cave_adapting: bool,
}

impl Creature {
    pub fn new(raw: &str, id: &str) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            names: Vec::new(),
            child_names: Vec::new(),
            description: String::new(),
            object_id: format!("{}-{}-{}", raw, "CREATURE", slugify(id)),
            lays_eggs: false,
            egg_size: 0,
            clutch_size: [0, 0],
            max_age: [0, 0],
            based_on: String::new(),

            adopts_owner: false,
            alcohol_dep: false,
            active_time: ActiveTimeKind::None,
            tiles: [' ', ' '],                    // tile and alt-tile
            aggression_kind: AggressionKind::None, // ambush_predator, large_predator, benign etc
            breathes_kind: BreathesKind::Air,
            arena_restricted: false, // true if not allowed spawning in arena
            artificial_hivable: false, // true if can be kept in hives by beekeeper
            peace_with_wildlife: false, // true prevents creature from attacking/frightening creatures that natural=true
            baby_until: 0,              // age at which becomes a child, def: 0
            baby_names: Vec::new(),     // Also should include GENERAL_BABY_NAME
            beach_frequency: 0, // freq of getting stranded on shores, e.g. orcas, sperm whales

            biome: Vec::new(),
            bloodsucker: false,
            body_size: Vec::new(),
            // Should have sub structs for body, body-parts
            diet: DietType::None,
            building_destroyer: 0, // 0 not, 1 targets doors, hatches, furn, 2 targets all non-constructions
            can_learn: false,
            can_speak: false,
            cannot_climb: false,
            cannot_jump: false,
            cannot_undead: false,
            can_open_doors: false,
            // Should have sub structs for caste
            cave_adapting: false,
        }
    }
}
