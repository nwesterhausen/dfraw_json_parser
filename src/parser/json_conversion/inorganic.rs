use crate::parser::raws::{inorganic::DFInorganic, material, tags::InorganicTag};

use super::TypedJsonSerializable;
use serde::{Deserialize, Serialize};

// Creature Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJsonInorganic {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    raw_module: String,
    raw_module_version: String,
    raw_module_found_in: String,
    raw_module_display: String,
    raw_type: String,
    #[serde(rename = "objectId")]
    object_id: String,
    tags: Vec<InorganicTag>,
    name: String,

    // Material
    material: material::SimpleMaterial,
}

impl TypedJsonInorganic {
    pub fn from(inorganic: &DFInorganic) -> Self {
        Self {
            identifier: inorganic.get_identifier(),
            parent_raw: inorganic.get_parent_raw(),
            object_id: inorganic.get_object_id(),
            raw_module: inorganic.get_raw_module(),
            raw_module_version: inorganic.get_raw_module_version(),
            raw_module_found_in: inorganic.get_dfraw_found_in(),
            raw_module_display: inorganic.get_dfraw_display(),
            raw_type: inorganic.get_raw_type(),
            name: inorganic.get_general_name(),

            material: inorganic.material.clone(),
            tags: Vec::new(),
        }
    }
}

impl TypedJsonSerializable for DFInorganic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJsonInorganic::from(&self))
    }
}
