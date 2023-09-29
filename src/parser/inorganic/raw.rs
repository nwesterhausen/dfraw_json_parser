use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    material::raw::Material,
    object_types::ObjectType,
    raws::{RawMetadata, RawObject},
    serializer_helper,
};

use super::{
    phf_table::{ENVIRONMENT_CLASS_TOKENS, INCLUSION_TYPE_TOKENS, INORGANIC_TOKENS},
    tokens::{EnvironmentClass, InclusionType, InorganicToken},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Inorganic {
    identifier: String,
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    object_id: String,
    material: Material,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    metal_ore_chance: Vec<(String, u8)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    thread_metal_chance: Vec<(String, u8)>,

    #[serde(skip_serializing_if = "EnvironmentClass::is_default")]
    environment_class: EnvironmentClass,
    #[serde(skip_serializing_if = "InclusionType::is_default")]
    environment_inclusion_type: InclusionType,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    environment_inclusion_frequency: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    environment_class_specific: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<InorganicToken>,
}

impl Inorganic {
    pub fn empty() -> Inorganic {
        Inorganic::default()
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Inorganic {
        Inorganic {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "INORGANIC",
                slugify(identifier)
            ),
            ..Inorganic::default()
        }
    }
}

#[typetag::serde]
impl RawObject for Inorganic {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }

    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }

    fn get_type(&self) -> &ObjectType {
        &ObjectType::Inorganic
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        if INORGANIC_TOKENS.contains_key(key) {
            // For the inorganic tokens, we need to check for (and parse) the MetalOre, ThreadMetal, Environment, and EnvironmentSpecific tokens.
            let token = INORGANIC_TOKENS
                .get(key)
                .unwrap_or(&InorganicToken::Unknown);

            match token {
                InorganicToken::Environment => {
                    // Environment values are like this: "class:type:frequency"
                    let mut split = value.split(':');
                    // Determine class
                    self.environment_class = ENVIRONMENT_CLASS_TOKENS
                        .get(split.next().unwrap_or(""))
                        .unwrap_or(&EnvironmentClass::None)
                        .clone();
                    // Determine type
                    self.environment_inclusion_type = INCLUSION_TYPE_TOKENS
                        .get(split.next().unwrap_or(""))
                        .unwrap_or(&InclusionType::None)
                        .clone();
                    // Determine frequency
                    self.environment_inclusion_frequency =
                        split.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                }
                InorganicToken::EnvironmentSpecific => {
                    self.environment_class_specific.push(String::from(value));
                }
                InorganicToken::MetalOre => {
                    // Metal ore token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
                    self.metal_ore_chance.push((metal, chance));
                }
                InorganicToken::ThreadMetal => {
                    // Thread metal token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
                    self.thread_metal_chance.push((metal, chance));
                }
                _ => {
                    self.tags.push(token.clone());
                }
            }

            return;
        }

        // Fall through any remaining tags to the material
        self.material.parse_tag(key, value);
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
