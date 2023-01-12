use crate::parser::raws::{
    environment, inorganic::DFInorganic, material, roll_chance, tags::InorganicTag,
    RawModuleLocation,
};

use super::TypedJsonSerializable;
use serde::{Deserialize, Serialize};

// Creature Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJsonInorganic {
    // Common Raw file Things
    identifier: String,
    #[serde(rename = "parentRaw")]
    parent_raw: String,
    #[serde(rename = "rawModule")]
    raw_module: String,
    #[serde(rename = "moduleVersion")]
    raw_module_version: String,
    #[serde(rename = "moduleSourceDirectory")]
    raw_module_found_in: RawModuleLocation,
    #[serde(rename = "moduleDisplayName")]
    raw_module_display: String,
    #[serde(rename = "rawType")]
    raw_type: String,
    #[serde(rename = "rawRelativePath")]
    relative_path: String,
    #[serde(rename = "objectId")]
    object_id: String,

    tags: Vec<InorganicTag>,
    name: String,

    // Material
    material: material::SimpleMaterial,
    environments: Vec<environment::Environment>,
    #[serde(rename = "specificEnvironments")]
    specific_environments: Vec<environment::Environment>,
    #[serde(rename = "metalOres")]
    metal_ores: Vec<roll_chance::RollChance>,
    #[serde(rename = "threadMetals")]
    thread_metals: Vec<roll_chance::RollChance>,

    // Calculated
    #[serde(rename = "magmaSafe")]
    magma_safe: bool,
}

impl TypedJsonInorganic {
    pub fn from(inorganic: &DFInorganic) -> Self {
        Self {
            identifier: inorganic.get_raw_header().get_identifier(),
            parent_raw: inorganic.get_raw_header().get_parent_raw(),
            object_id: inorganic.get_raw_header().get_object_id(),
            raw_module: inorganic.get_raw_header().get_raw_module(),
            raw_module_version: inorganic.get_raw_header().get_raw_module_version(),
            raw_module_found_in: inorganic.get_raw_header().get_dfraw_found_in(),
            raw_module_display: inorganic.get_raw_header().get_dfraw_display(),
            relative_path: inorganic.get_raw_header().get_dfraw_relative_path(),
            raw_type: inorganic.get_raw_header().get_raw_type(),
            name: inorganic.get_general_name(),

            material: inorganic.material.clone(),
            tags: inorganic.tags.clone(),
            environments: inorganic.environments.clone(),
            specific_environments: inorganic.environments_specific.clone(),
            metal_ores: inorganic.metal_ores.clone(),
            thread_metals: inorganic.thread_metals.clone(),

            magma_safe: inorganic.is_magma_safe(),
        }
    }
}

impl TypedJsonSerializable for DFInorganic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJsonInorganic::from(self))
    }
}

impl TypedJsonSerializable for &DFInorganic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJsonInorganic::from(self))
    }
}
