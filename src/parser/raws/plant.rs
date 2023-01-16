use std::collections::HashMap;

use crate::parser::raws::{
    info::DFInfoFile,
    names::Name,
    tags::{self},
};
use crate::parser::raws::{material, names};

use serde::{Deserialize, Serialize};

use super::DFRawCommon;

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

impl DFPlant {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            raw_header: DFRawCommon::from(id, raw, info_text, super::RawObjectKind::Plant),
            // Boolean Flags
            tags: Vec::new(),

            // integers
            frequency: 50, //Defaults to 50 if not specified
            cluster_size: 0,

            biomes: Vec::new(),
            name: Name::new(""),

            pref_string: Vec::new(),
            value: 0,
            underground_depth: [0, 0],
            growth_duration: 0,
            growth_names: HashMap::new(),

            // Simple materials
            materials_vec: Vec::new(),

            // Simple reactions..
            reactions: Vec::new(),
        }
    }
    pub fn get_raw_header(&self) -> &DFRawCommon {
        &self.raw_header
    }
    pub fn set_overwrites_raw(&mut self, raw_name: &str) {
        self.raw_header.overwrites_raw = String::from(raw_name);
    }
    pub fn push_cut_tag(&mut self, tag0: &str, tag1: &str) {
        self.raw_header.push_cut_tag(tag0, tag1);
    }
    pub fn get_general_name(&self) -> String {
        if !self.get_raw_header().overwrites_raw.is_empty() {
            return format!("Overwrite {}", self.get_raw_header().overwrites_raw);
        }
        self.name.to_string_vec()[0].to_string()
    }
}
