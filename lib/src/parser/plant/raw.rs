use serde::{Deserialize, Serialize};
use slug::slugify;
use tracing::{debug, warn};

use crate::parser::{
    biome,
    helpers::parse_min_max_range,
    material::{Material, PROPERTY_TOKEN_MAP, USAGE_TOKEN_MAP},
    plant_growth::{
        PlantGrowth, Token as GrowthToken, TypeToken as GrowthTypeToken,
        TOKEN_MAP as GROWTH_TOKEN_MAP, TYPE_TOKEN_MAP as GROWTH_TYPE_TOKEN_MAP,
    },
    serializer_helper,
    shrub::{Shrub, TOKEN_MAP as SHRUB_TOKEN_MAP},
    tree::{Tree, TOKEN_MAP as TREE_TOKEN_MAP},
    Name, ObjectType, {clean_search_vec, Searchable}, {RawMetadata, RawObject},
};

use super::{phf_table::PLANT_TOKENS, tokens::PlantTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Plant {
    /// Common Raw file Things
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    // Basic Tokens
    name: Name,
    pref_strings: Option<Vec<String>>,
    tags: Option<Vec<PlantTag>>,

    // Environment Tokens
    /// Default [0, 0] (aboveground)
    //#[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    underground_depth: Option<[u32; 2]>,
    /// Default frequency is 50
    //#[serde(skip_serializing_if = "serializer_helper::is_default_frequency")]
    frequency: Option<u32>,
    /// List of biomes this plant can grow in
    biomes: Option<Vec<biome::Token>>,

    /// Growth Tokens define the growths of the plant (leaves, fruit, etc.)
    growths: Option<Vec<PlantGrowth>>,
    /// If plant is a tree, it will have details about the tree.
    tree_details: Option<Tree>,
    /// If plant is a shrub, it will have details about the shrub.
    shrub_details: Option<Shrub>,

    materials: Option<Vec<Material>>,
}

impl Plant {
    pub fn empty() -> Plant {
        Plant {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::Plant)
                    .with_hidden(true),
            ),
            frequency: Some(50),
            ..Plant::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Plant {
        Plant {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            frequency: Some(50),
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "PLANT",
                slugify(identifier)
            ),
            ..Plant::default()
        }
    }
    pub fn get_biomes(&self) -> Vec<biome::Token> {
        if let Some(biomes) = &self.biomes {
            biomes.clone()
        } else {
            Vec::new()
        }
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if is_metadata_hidden is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if is_metadata_hidden is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        if let Some(pref_strings) = &cleaned.pref_strings {
            if pref_strings.is_empty() {
                cleaned.pref_strings = None;
            }
        }

        if let Some(tags) = &cleaned.tags {
            if tags.is_empty() {
                cleaned.tags = None;
            }
        }

        if serializer_helper::min_max_is_zeroes(&cleaned.underground_depth) {
            cleaned.underground_depth = None;
        }

        if serializer_helper::is_default_frequency(&cleaned.frequency) {
            cleaned.frequency = None;
        }

        if let Some(biomes) = &cleaned.biomes {
            if biomes.is_empty() {
                cleaned.biomes = None;
            }
        }

        if let Some(growths) = &cleaned.growths {
            let mut cleaned_growths = Vec::new();
            for growth in growths {
                cleaned_growths.push(growth.cleaned());
            }
            cleaned.growths = Some(cleaned_growths);
        }

        if let Some(materials) = &cleaned.materials {
            let mut cleaned_materials = Vec::new();
            for material in materials {
                cleaned_materials.push(material.cleaned());
            }
            if cleaned_materials.is_empty() {
                cleaned.materials = None;
            }
            cleaned.materials = Some(cleaned_materials);
        }

        cleaned
    }
    /// Add a tag to the plant.
    ///
    /// This handles making sure the tags vector is initialized.
    pub fn add_tag(&mut self, tag: PlantTag) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag);
        } else {
            warn!(
                "Plant::add_tag: ({}) Failed to add tag {:?}",
                self.identifier, tag
            );
        }
    }
}

#[typetag::serde]
impl RawObject for Plant {
    fn get_metadata(&self) -> &RawMetadata {
        if let Some(metadata) = &self.metadata {
            metadata
        } else {
            warn!(
                "PlantParsing: Failed to get metadata for plant {}",
                self.identifier
            );
            &RawMetadata::default()
                .with_object_type(ObjectType::Plant)
                .with_hidden(true)
        }
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        self.name.get_singular()
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }

    fn clean_self(&mut self) {
        *self = self.cleaned();
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::Plant
    }
    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if (PROPERTY_TOKEN_MAP.contains_key(key) || USAGE_TOKEN_MAP.contains_key(key))
            && !key.eq("USE_MATERIAL_TEMPLATE")
        {
            // have our latest material parse the tag
            if let Some(materials) = self.materials.as_mut() {
                if let Some(material) = materials.last_mut() {
                    material.parse_tag(key, value);
                } else {
                    warn!(
                        "PlantParsing: Failed to find material to add tag {} with value {}",
                        key, value
                    );
                }
            }
            return;
        }

        if TREE_TOKEN_MAP.contains_key(key) {
            if self.tree_details.is_none() {
                self.tree_details = Some(Tree::new(value));
            }
            let tree = self.tree_details.as_mut().unwrap();
            tree.parse_tag(key, value);
            return;
        }

        if GROWTH_TOKEN_MAP.contains_key(key) {
            if self.growths.is_none() {
                self.growths = Some(Vec::new());
            }
            let token = GROWTH_TOKEN_MAP.get(key).unwrap_or(&GrowthToken::Unknown);
            if token == &GrowthToken::Growth {
                // If we are defining a new growth, we need to create a new PlantGrowth
                let growth_type = GROWTH_TYPE_TOKEN_MAP
                    .get(value)
                    .unwrap_or(&GrowthTypeToken::None)
                    .clone();
                let growth = PlantGrowth::new(growth_type);
                if let Some(growths) = self.growths.as_mut() {
                    growths.push(growth);
                }
                return;
            }
            // Otherwise, we are defining a tag for the current growth (most recently added)
            if let Some(growths) = self.growths.as_mut() {
                if let Some(growth) = growths.last_mut() {
                    growth.parse_tag(key, value);
                } else {
                    warn!(
                        "PlantParsing: Failed to find growth to add tag {} with value {}",
                        key, value
                    );
                }
            }
            return;
        }

        if SHRUB_TOKEN_MAP.contains_key(key) {
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
            debug!("PlantParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = PLANT_TOKENS.get(key) else {
            warn!(
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
                if self.pref_strings.is_none() {
                    self.pref_strings = Some(Vec::new());
                }
                if let Some(pref_strings) = &mut self.pref_strings {
                    pref_strings.push(String::from(value));
                }
            }
            PlantTag::Biome => {
                let Some(biome) = biome::TOKEN_MAP.get(value) else {
                    warn!(
                        "PlantParsing: called `Option::unwrap()` on a `None` value for presumed biome: {}",
                        value
                    );
                    return;
                };
                if self.biomes.is_none() {
                    self.biomes = Some(Vec::new());
                }
                if let Some(biomes) = &mut self.biomes {
                    biomes.push(biome.clone());
                }
            }
            PlantTag::UndergroundDepth => {
                self.underground_depth = Some(parse_min_max_range(value).unwrap_or([0, 0]));
            }
            PlantTag::Frequency => {
                self.frequency = Some(value.parse::<u32>().unwrap_or(50));
            }
            PlantTag::UseMaterialTemplate => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_template_from_value(value));
                }
            }
            PlantTag::UseMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_from_value(value));
                }
            }
            PlantTag::BasicMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::basic_material_from_value(value));
                }
            }
            PlantTag::Material => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::from_value(value));
                }
            }
            _ => {
                self.add_tag(tag.clone());
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
        if let Some(pref_strings) = &self.pref_strings {
            vec.extend(pref_strings.clone());
        }
        if let Some(biomes) = &self.biomes {
            vec.extend(biomes.iter().map(std::string::ToString::to_string));
        }
        if let Some(tags) = &self.tags {
            vec.extend(tags.iter().map(std::string::ToString::to_string));
        }
        if let Some(growths) = &self.growths {
            vec.extend(growths.iter().flat_map(Searchable::get_search_vec));
        }
        if let Some(materials) = &self.materials {
            vec.extend(materials.iter().flat_map(Searchable::get_search_vec));
        }

        clean_search_vec(vec.as_slice())
    }
}
