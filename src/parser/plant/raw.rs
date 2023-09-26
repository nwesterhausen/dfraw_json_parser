use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    names::Name,
    object_types::ObjectType,
    ranges::parse_min_max_range,
    ranges::Ranges,
    raws::{RawMetadata, RawObject},
};

use super::{phf_table::PLANT_TOKENS, tokens::PlantTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DFPlant {
    /// Common Raw file Things
    // #[serde(skip_serializing_if = "RawMetadata::is_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    // Basic Tokens
    name: Name,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pref_strings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<PlantTag>,

    // Environment Tokens
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    underground_depth: [u16; 2],
    /// Default frequency is 50
    #[serde(skip_serializing_if = "Ranges::is_default_frequency")]
    frequency: u16,
    /// List of biomes this plant can grow in
    #[serde(skip_serializing_if = "Vec::is_empty")]
    biomes: Vec<String>,
}

impl DFPlant {
    pub fn empty() -> DFPlant {
        DFPlant {
            frequency: 50,
            ..DFPlant::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> DFPlant {
        DFPlant {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            frequency: 50,
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "PLANT",
                slugify(identifier)
            ),
            ..DFPlant::default()
        }
    }
}

#[typetag::serde]
impl RawObject for DFPlant {
    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }

    fn get_type(&self) -> &ObjectType {
        &ObjectType::Plant
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        if !PLANT_TOKENS.contains_key(key) {
            log::debug!("PlantParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = PLANT_TOKENS.get(key) else {
            log::warn!(
                "PlantParsing: called `Option::unwrap()` on a `None` value for presumed plant tag: {}",
                key
            );
            return;
        };

        match tag {
            PlantTag::NameSingular => {
                self.name.update_singular(value);
            }
            PlantTag::NamePlural => {
                self.name.update_plural(value);
            }
            PlantTag::NameAdjective => {
                self.name.update_adjective(value);
            }
            PlantTag::AllNames => {
                self.name = Name::from_value(value);
            }
            PlantTag::PrefString => {
                self.pref_strings.push(String::from(value));
            }
            PlantTag::Biome => {
                self.biomes.push(String::from(value));
            }
            PlantTag::UndergroundDepth => {
                self.underground_depth = parse_min_max_range(value).unwrap_or([0, 0]);
            }
            PlantTag::Frequency => {
                self.frequency = value.parse::<u16>().unwrap_or(50);
            }
            _ => {
                self.tags.push(tag.clone());
            }
        }
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
