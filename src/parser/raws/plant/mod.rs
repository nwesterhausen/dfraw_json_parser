use std::collections::HashMap;

use crate::parser::raws::{material, names};
use crate::parser::raws::{
    names::Name,
    tags::{self},
};

use serde::{Deserialize, Serialize};

use super::DFRawCommon;

mod parse;
mod plant_basic;
mod typed_json;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DFPlant {
    /// Common Raw file Things
    raw_header: DFRawCommon,
    pub tags: Vec<tags::PlantTag>,

    // Basic Tokens
    pub name: Name,
    pub pref_string: Vec<String>,
    pub value: u32,
    pub growth_duration: u32,
    pub growth_names: HashMap<Growth, names::SingPlurName>,

    // Environment Tokens
    pub underground_depth: [u32; 2],
    pub frequency: u32,
    pub cluster_size: u32,
    pub biomes: Vec<String>,
    // pub growth: DFPlantGrowth,
    // pub materials: Vec<DFMaterialTemplate>,
    // pub seed: DFPlantSeed,
    // Sub Tags
    pub materials_vec: Vec<material::SimpleMaterial>,
    pub reactions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Growth {
    None,
    Leaves,
    Spathes,
    Fruit,
    Flowers,
    Nut,
    SeedCatkins,
    PollenCatkins,
    Cone,
    SeedCone,
    PollenCone,
    Feathers,
    Eggs,
    Pod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantSeed {
    pub name: Name,
}
