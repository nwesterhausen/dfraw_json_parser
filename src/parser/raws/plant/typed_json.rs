use std::collections::HashMap;

use crate::parser::{
    raws::{
        material::SimpleMaterial,
        names::SingPlurName,
        plant::{DFPlant, Growth},
        tags::PlantTag,
        RawModuleLocation,
    },
    TypedJsonSerializable,
};

use serde::{Deserialize, Serialize};

// Creature Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypedJson {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    raw_module: String,
    raw_module_version: String,
    raw_module_found_in: RawModuleLocation,
    raw_module_display: String,
    raw_type: String,
    relative_path: String,
    object_id: String,
    overwrites_raw: String,

    tags: Vec<PlantTag>,
    name: String,

    // Basic Tokens
    pref_string: Vec<String>,
    value: u32,

    // Environment Tokens
    underground_depth: [u32; 2],
    frequency: u32,
    #[serde(rename = "clusterSize")]
    cluster_size: u32,
    biomes: Vec<String>,
    // pub growth: DFPlantGrowth,
    // pub materials: Vec<DFMaterialTemplate>,
    // pub seed: DFPlantSeed,
    // Sub Tags
    materials: Vec<SimpleMaterial>,
    growth_names: HashMap<Growth, SingPlurName>,
    growth_duration: u32,

    reactions: Vec<String>,
}

impl TypedJson {
    pub fn from(plant: &DFPlant) -> Self {
        Self {
            identifier: plant.get_raw_header().get_identifier(),
            parent_raw: plant.get_raw_header().get_parent_raw(),
            object_id: plant.get_raw_header().get_object_id(),
            raw_module: plant.get_raw_header().get_raw_module(),
            raw_module_version: plant.get_raw_header().get_raw_module_version(),
            raw_module_found_in: plant.get_raw_header().get_dfraw_found_in(),
            raw_module_display: plant.get_raw_header().get_dfraw_display(),
            relative_path: plant.get_raw_header().get_dfraw_relative_path(),
            raw_type: plant.get_raw_header().get_raw_type(),
            name: plant.get_general_name(),
            overwrites_raw: plant.get_raw_header().overwrites_raw.to_string(),

            frequency: plant.frequency,
            // cloning vec
            pref_string: plant.pref_string.clone(),
            tags: Vec::clone(&plant.tags),
            biomes: Vec::clone(&plant.biomes),
            cluster_size: plant.cluster_size,
            underground_depth: plant.underground_depth,
            value: plant.value,
            materials: Vec::clone(&plant.materials_vec),

            growth_names: HashMap::clone(&plant.growth_names),
            growth_duration: plant.growth_duration,

            reactions: Vec::clone(&plant.reactions),
        }
    }
}

impl TypedJsonSerializable for DFPlant {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}

impl TypedJsonSerializable for &DFPlant {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}
