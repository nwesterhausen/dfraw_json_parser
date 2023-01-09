use std::collections::HashMap;

use crate::parser::raws::{
    info::DFInfoFile,
    names::Name,
    tags::{self},
};
use crate::parser::raws::{material, names};
use crate::parser::reader::RawObjectKind;
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Debug)]
pub struct DFPlant {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    dfraw_identifier: String,
    dfraw_version: String,
    dfraw_found_in: String,
    dfraw_display: String,
    dfraw_relative_path: String,
    raw_type: RawObjectKind,
    pub tags: Vec<tags::PlantTag>,

    // Basic Tokens
    pub name: Name,
    pub pref_string: Vec<String>,
    pub value: u32,
    pub growth_duration: u32,
    pub growth_names: HashMap<PlantGrowth, names::SingPlurName>,

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
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlantGrowth {
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
            identifier: String::from(id),
            parent_raw: String::from(raw),
            dfraw_identifier: String::from(info_text.get_identifier()),
            dfraw_version: String::from(info_text.displayed_version.as_str()),
            dfraw_found_in: String::from(info_text.get_sourced_directory()),
            dfraw_display: format!("{} v{}", info_text.name, info_text.displayed_version),
            dfraw_relative_path: String::from(info_text.get_relative_path()),
            raw_type: RawObjectKind::Plant,
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
        }
    }

    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_raw_module(&self) -> String {
        String::from(&self.dfraw_identifier)
    }
    pub fn get_raw_module_version(&self) -> String {
        String::from(&self.dfraw_version)
    }
    pub fn get_dfraw_found_in(&self) -> String {
        String::from(&self.dfraw_found_in)
    }
    pub fn get_dfraw_display(&self) -> String {
        String::from(&self.dfraw_display)
    }
    pub fn get_dfraw_relative_path(&self) -> String {
        String::from(&self.dfraw_relative_path)
    }
    pub fn get_parent_raw(&self) -> String {
        String::from(&self.parent_raw)
    }
    pub fn get_raw_type(&self) -> String {
        format!("{:?}", self.raw_type)
    }
    pub fn get_object_id(&self) -> String {
        format!(
            "{}-{}-{}",
            self.get_parent_raw(),
            "PLANT",
            slugify(self.get_identifier())
        )
    }
    pub fn get_general_name(&self) -> String {
        self.name.to_string_vec()[0].to_string()
    }
}
