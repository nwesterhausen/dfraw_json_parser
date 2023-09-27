use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    names::Name,
    object_types::ObjectType,
    plant_growth::{
        phf_table::{GROWTH_TOKENS, GROWTH_TYPE_TOKENS},
        raw::PlantGrowth,
        tokens::{GrowthTag, GrowthType},
    },
    ranges::parse_min_max_range,
    ranges::Ranges,
    raws::{RawMetadata, RawObject},
    tree::{phf_table::TREE_TOKENS, raw::Tree},
};

use super::{phf_table::PLANT_TOKENS, tokens::PlantTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DFPlant {
    /// Common Raw file Things
    #[serde(skip_serializing_if = "RawMetadata::is_hidden")]
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

    /// Growth Tokens define the growths of the plant (leaves, fruit, etc.)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    growths: Vec<PlantGrowth>,
    /// If plant is a tree, it will have details about the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_details: Option<Tree>,

    // Todo fix later
    #[serde(skip_serializing_if = "Vec::is_empty")]
    material_templates: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    materials: Vec<String>,
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
        if TREE_TOKENS.contains_key(key) {
            if self.tree_details.is_none() {
                self.tree_details = Some(Tree::new(value));
            }
            let tree = self.tree_details.as_mut().unwrap();
            tree.parse_tag(key, value);
            return;
        }

        if GROWTH_TOKENS.contains_key(key) {
            let token = GROWTH_TOKENS.get(key).unwrap_or(&GrowthTag::Unknown);
            if token == &GrowthTag::Growth {
                // If we are defining a new growth, we need to create a new PlantGrowth
                let growth_type = GROWTH_TYPE_TOKENS
                    .get(value)
                    .unwrap_or(&GrowthType::None)
                    .clone();
                let growth = PlantGrowth::new(growth_type);
                self.growths.push(growth);
                return;
            }
            // Otherwise, we are defining a tag for the current growth (most recently added)
            self.growths
                .last_mut()
                .unwrap_or(&mut PlantGrowth::default())
                .parse_tag(key, value);
            return;
        }

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
            PlantTag::UseMaterialTemplate => {
                self.material_templates.push(String::from(value));
            }
            PlantTag::UseMaterial => {
                self.materials.push(String::from(value));
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
