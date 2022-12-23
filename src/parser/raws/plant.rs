use crate::parser::raws::{
    info::DFInfoFile,
    names::{Name, SingPlurName},
    tags::{self, CasteTag},
};
use crate::parser::reader::RawObjectKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DFCreature {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    dfraw_identifier: String,
    dfraw_version: String,
    dfraw_found_in: String,
    raw_type: RawObjectKind,
    #[serde(rename = "objectId")]
    object_id: String,
    pub tags: Vec<tags::PlantTag>,

    // Basic Tokens
    pub name: Name,
    pub pref_string: Vec<String>,
    pub value: u32,

    // Environment Tokens
    pub underground_depth: [u32; 2],
    pub frequency: u32,
    pub cluster_size: u32,
    pub biomes: Vec<String>,

    pub growth: DFPlantGrowth,
    pub materials: Vec<DFMaterialTemplate>,
    pub seed: DFPlantSeed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantGrowth {
    pub name: Name,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantSeed {
    pub name: Name,
}
