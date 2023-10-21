use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    material::{
        phf_table::{MATERIAL_PROPERTY_TOKENS, MATERIAL_USAGE_TOKENS},
        raw::Material,
    },
    names::Name,
    object_types::ObjectType,
    plant_growth::{
        phf_table::{GROWTH_TOKENS, GROWTH_TYPE_TOKENS},
        raw::PlantGrowth,
        tokens::{GrowthTag, GrowthType},
    },
    ranges::parse_min_max_range,
    raws::{RawMetadata, RawObject},
    searchable::{clean_search_vec, Searchable},
    serializer_helper,
    shrub::{phf_table::SHRUB_TOKENS, raw::Shrub},
    tree::{phf_table::TREE_TOKENS, raw::Tree},
};

use super::{phf_table::PLANT_TOKENS, tokens::PlantTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Plant {
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
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    underground_depth: [u16; 2],
    /// Default frequency is 50
    #[serde(skip_serializing_if = "serializer_helper::is_default_frequency")]
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
    /// If plant is a shrub, it will have details about the shrub.
    #[serde(skip_serializing_if = "Option::is_none")]
    shrub_details: Option<Shrub>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    materials: Vec<Material>,
}

impl Plant {
    pub fn empty() -> Plant {
        Plant {
            frequency: 50,
            ..Plant::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Plant {
        Plant {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            frequency: 50,
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "PLANT",
                slugify(identifier)
            ),
            ..Plant::default()
        }
    }
}

#[typetag::serde]
impl RawObject for Plant {
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
    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if (MATERIAL_PROPERTY_TOKENS.contains_key(key) || MATERIAL_USAGE_TOKENS.contains_key(key))
            && !key.eq("USE_MATERIAL_TEMPLATE")
        {
            // have our latest material parse the tag
            self.materials
                .last_mut()
                .unwrap_or(&mut Material::default())
                .parse_tag(key, value);
            return;
        }

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

        if SHRUB_TOKENS.contains_key(key) {
            if self.shrub_details.is_none() {
                self.shrub_details = Some(Shrub::new());
            }
            self.shrub_details
                .as_mut()
                .unwrap_or(&mut Shrub::default())
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
                self.materials
                    .push(Material::use_material_template_from_value(value));
            }
            PlantTag::UseMaterial => {
                self.materials
                    .push(Material::use_material_from_value(value));
            }
            PlantTag::BasicMaterial => {
                self.materials
                    .push(Material::basic_material_from_value(value));
            }
            PlantTag::Material => {
                self.materials.push(Material::from_value(value));
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

impl Searchable for Plant {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.extend(self.name.as_vec());
        vec.extend(self.pref_strings.clone());
        vec.extend(self.biomes.clone());
        vec.extend(self.tags.iter().map(|tag| format!("{tag:?}")));
        vec.extend(
            self.growths
                .iter()
                .flat_map(Searchable::get_search_vec)
                .collect::<Vec<String>>(),
        );
        vec.extend(
            self.materials
                .iter()
                .flat_map(Searchable::get_search_vec)
                .collect::<Vec<String>>(),
        );

        clean_search_vec(vec.as_slice())
    }
}
