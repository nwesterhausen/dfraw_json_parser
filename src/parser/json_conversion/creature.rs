use std::collections::HashMap;

use super::TypedJsonSerializable;
use crate::parser::raws::creature::DFCreature;
use crate::parser::raws::tags::{CasteTag, CreatureTag, DFBodySize, DFMilkable};
use crate::parser::raws::RawModuleLocation;
use serde::{Deserialize, Serialize};

// Creature Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJson {
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

    name: String,
    tags: Vec<CreatureTag>,

    #[serde(rename = "namesMap")]
    names_map: HashMap<String, Vec<String>>,
    descriptions: HashMap<String, String>,
    #[serde(rename = "maxAge")]
    max_age: HashMap<String, [u16; 2]>,
    #[serde(rename = "clutchSize")]
    clutch_size: HashMap<String, [u16; 2]>,
    #[serde(rename = "basedOn")]
    based_on: String,
    biomes: Vec<String>,
    #[serde(rename = "clusterRange")]
    cluster_range: [u16; 2],
    #[serde(rename = "undergroundDepth")]
    underground_depth: [u16; 2],
    #[serde(rename = "bodySize")]
    body_size: HashMap<String, Vec<DFBodySize>>,
    #[serde(rename = "grownAt")]
    grown_at: HashMap<String, u32>,
    #[serde(rename = "childAt")]
    child_at: HashMap<String, u32>,
    #[serde(rename = "eggSizes")]
    egg_sizes: HashMap<String, u32>,
    #[serde(rename = "petValue")]
    pet_value: HashMap<String, u16>,
    intelligence: HashMap<String, [bool; 2]>,
    flier: HashMap<String, bool>,
    gnawer: HashMap<String, bool>,
    trainable: HashMap<String, u8>,
    #[serde(rename = "activeTime")]
    active_time: HashMap<String, u8>,
    #[serde(rename = "inactiveSeason")]
    inactive_season: HashMap<String, u8>,
    #[serde(rename = "creatureClass")]
    creature_class: HashMap<String, Vec<String>>,
    #[serde(rename = "casteTags")]
    caste_tags: HashMap<String, Vec<CasteTag>>,
    difficulty: HashMap<String, u32>,
    #[serde(rename = "grassTrample")]
    grass_trample: HashMap<String, u8>,
    grazer: HashMap<String, u32>,
    #[serde(rename = "lowlightVision")]
    lowlight_vision: HashMap<String, u32>,
    #[serde(rename = "populationRatio")]
    population_ratio: HashMap<String, u16>,
    milkable: HashMap<String, DFMilkable>,
    #[serde(rename = "preferenceStrings")]
    pref_string: Vec<String>,
    #[serde(rename = "populationNumber")]
    population_number: [u16; 2],
}

impl TypedJson {
    pub fn from(creature: &DFCreature) -> Self {
        Self {
            identifier: creature.get_raw_header().get_identifier(),
            parent_raw: creature.get_raw_header().get_parent_raw(),
            object_id: creature.get_raw_header().get_object_id(),
            raw_module: creature.get_raw_header().get_raw_module(),
            raw_module_version: creature.get_raw_header().get_raw_module_version(),
            raw_module_found_in: creature.get_raw_header().get_dfraw_found_in(),
            raw_module_display: creature.get_raw_header().get_dfraw_display(),
            raw_type: creature.get_raw_header().get_raw_type(),
            relative_path: creature.get_raw_header().get_dfraw_relative_path(),
            name: creature.get_general_name(),
            descriptions: creature.get_description_by_caste(),
            max_age: creature.get_max_ages_by_caste(),
            clutch_size: creature.get_clutch_sizes_by_caste(),
            based_on: creature.copy_tags_from.join(""),
            biomes: Vec::clone(&creature.biomes),
            cluster_range: creature.cluster_number,
            underground_depth: creature.underground_depth,
            body_size: creature.get_body_sizes_by_caste(),
            grown_at: creature.get_grown_at_ages_by_caste(),
            names_map: creature.get_names_by_caste(),
            egg_sizes: creature.get_egg_sizes_by_caste(),
            pet_value: creature.get_pet_value_by_caste(),
            intelligence: creature.get_intelligence_by_caste(),
            flier: creature.get_flier_by_caste(),
            gnawer: creature.get_gnawer_by_caste(),
            trainable: creature.get_trainable_by_caste(),
            active_time: creature.get_active_time_by_caste(),
            inactive_season: creature.get_inactive_season_by_caste(),
            creature_class: creature.get_creature_class_by_caste(),
            tags: Vec::clone(&creature.tags),
            caste_tags: creature.get_caste_tags(),
            difficulty: creature.get_difficulty_by_caste(),
            grass_trample: creature.get_grass_trample_by_caste(),
            grazer: creature.get_grazer_by_caste(),
            child_at: creature.get_child_ages_by_caste(),
            lowlight_vision: creature.get_low_light_vision_by_caste(),
            milkable: creature.get_milkable_by_caste(),
            population_ratio: creature.get_pop_ratio_by_caste(),
            pref_string: creature.pref_string.clone(),
            population_number: creature.population_number,
        }
    }
}

impl TypedJsonSerializable for DFCreature {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}

impl TypedJsonSerializable for &DFCreature {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}
