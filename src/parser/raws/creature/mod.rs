use crate::parser::raws::names::{Name, SingPlurName};

mod caste_basic;
mod creature_basic;
mod parse;
mod typed_json;

use serde::{Deserialize, Serialize};

use crate::parser::raws::DFRawCommon;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DFCreature {
    pub(crate) raw_header: DFRawCommon,

    // Boolean Flags
    pub tags: Vec<super::tags::CreatureTag>,

    // integers
    pub frequency: u16, //Defaults to 50 if not specified

    // [min, max] ranges
    pub cluster_number: [u16; 2],    //Defaults to 1:1 if not specified.
    pub population_number: [u16; 2], //default 1:1
    pub underground_depth: [u16; 2], //default 0:0 (aboveground)

    // strings
    pub general_baby_name: SingPlurName,
    pub general_child_name: SingPlurName,
    pub name: Name,

    // Arrays
    pub biomes: Vec<String>,
    pub pref_string: Vec<String>,

    // sub definitions
    pub castes: Vec<DFCreatureCaste>,

    // copy_from
    pub copy_tags_from: Vec<String>, // vec of creature identifiers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFCreatureCaste {
    // Identification
    pub name: String,
    // Boolean Flags
    pub tags: Vec<super::tags::CasteTag>,

    // [min, max] ranges
    pub clutch_size: [u16; 2],
    pub litter_size: [u16; 2],
    pub max_age: [u16; 2],

    // Combo flags (custom)
    pub active_time: u8, // MATUTINAL/DIURNAL/NOCTURNAL/CREPUSCULAR/VESPERTINE via binary math
    pub curious_beast: u8, // EATER/GUZZLER/ITEM via binary math
    pub no_season: u8,   // NO_SPRING/NO_SUMMER/NO_AUTUMN/NO_WINTER
    pub trainable: u8,   // trainable_HUNTING/trainable_WAR/BOTH(aka trainable)

    // Integer tokens
    pub baby: u32,
    pub child: u32,
    pub difficulty: u32,
    pub egg_size: u32,
    pub grass_trample: u8,
    pub grazer: u32,
    pub low_light_vision: u32,
    pub pet_value: u16,
    pub pop_ratio: u16,

    // String Tokens
    pub baby_name: SingPlurName,
    pub caste_name: Name,
    pub child_name: SingPlurName,
    pub description: String,

    // Arrays
    pub creature_class: Vec<String>,

    // Custom tokens
    pub body_size: Vec<super::tags::DFBodySize>,
    pub milkable: super::tags::DFMilkable,
}
