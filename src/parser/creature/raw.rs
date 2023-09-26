use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    creature_caste::{phf_table::CASTE_TOKENS, raw::DFCaste},
    creature_variation::raw::CreatureVariationRequirements,
    names::{Name, SingPlurName},
    object_types::ObjectType,
    ranges::parse_min_max_range,
    ranges::Ranges,
    raws::{RawMetadata, RawObject},
    tile::DFTile,
};

use super::{phf_table::CREATURE_TOKENS, tokens::CreatureTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DFCreature {
    // #[serde(skip_serializing_if = "RawMetadata::is_hidden")]
    metadata: RawMetadata,
    identifier: String,
    castes: Vec<DFCaste>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<CreatureTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    biomes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pref_strings: Vec<String>,
    #[serde(skip_serializing_if = "DFTile::is_default")]
    tile: DFTile,
    // integers
    #[serde(skip_serializing_if = "Ranges::is_default_frequency")]
    frequency: u16, //Defaults to 50 if not specified
    // [min, max] ranges
    /// Default [1, 1]
    #[serde(skip_serializing_if = "Ranges::min_max_is_ones")]
    cluster_number: [u16; 2],
    /// Default [1, 1]
    #[serde(skip_serializing_if = "Ranges::min_max_is_ones")]
    population_number: [u16; 2],
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    underground_depth: [u16; 2],
    // strings
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    general_baby_name: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    general_child_name: SingPlurName,
    name: Name,
    // Special tokens
    #[serde(skip_serializing_if = "String::is_empty")]
    copy_tags_from: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    apply_creature_variation: Vec<String>,
    object_id: String,
}

impl DFCreature {
    pub fn empty() -> DFCreature {
        DFCreature {
            castes: vec![DFCaste::new("ALL")],
            population_number: [1, 1],
            cluster_number: [1, 1],
            frequency: 50,
            ..DFCreature::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> DFCreature {
        DFCreature {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            frequency: 50,
            castes: vec![DFCaste::new("ALL")],
            population_number: [1, 1],
            cluster_number: [1, 1],
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "CREATURE",
                slugify(identifier)
            ),
            ..DFCreature::default()
        }
    }
    pub fn get_copy_tags_from(&self) -> &str {
        &self.copy_tags_from
    }
    // Add a new caste
    pub fn add_caste(&mut self, name: &str) {
        self.castes.push(DFCaste::new(name));
    }
    // Move specified caste (by name) to end of case list
    pub fn select_caste(&mut self, name: &str) {
        // Find the caste
        let mut index = 0;
        for (i, caste) in self.castes.iter().enumerate() {
            if caste.get_identifier().eq(name) {
                index = i;
                break;
            }
        }
        // If the caste is not found, add a new one
        if index == 0 && !self.castes.get(0).unwrap().get_identifier().eq(name) {
            return self.add_caste(name);
        }
        // Move the caste to the end of the list
        let caste = self.castes.remove(index);
        self.castes.push(caste);
    }

    pub fn has_caste(&self, name: &str) -> bool {
        for caste in &self.castes {
            if caste.get_identifier().eq(name) {
                return true;
            }
        }
        false
    }

    /// Copy tags from another creature
    pub fn copy_tags_from(creature: &DFCreature, creature_to_copy_from: &DFCreature) -> Self {
        // Because anything specified in our self will override the copied tags, first we need to clone the creature
        let mut combined_creature = creature_to_copy_from.clone();
        // Now apply any tags that exist for us but not for the one we copy.
        // So we need to go through all our properties and castes and overwrite what exists on the combined creature.

        // our metadata is preserved
        combined_creature.metadata = creature.metadata.clone();
        // our identifier is preserved
        combined_creature.identifier = creature.identifier.clone();

        // We need to loop over our castes and apply any differences.
        for caste in &creature.castes {
            let caste_identifier = caste.get_identifier();
            // If the caste exists in the combined creature, we need to apply the differences
            if combined_creature.has_caste(caste_identifier) {
                combined_creature.select_caste(caste_identifier);
                let combined_caste = combined_creature.castes.last_mut().unwrap();
                combined_caste.overwrite_caste(caste);
            } else {
                // If the caste does not exist in the combined creature, we need to add it
                combined_creature.castes.push(caste.clone());
            }
        }

        // Loop over our pref_strings and if they aren't in combined_creature, add them
        for pref_string in &creature.pref_strings {
            if !combined_creature.pref_strings.contains(pref_string) {
                combined_creature.pref_strings.push(pref_string.clone());
            }
        }

        // Loop over our biomes and if they aren't in combined_creature, add them
        for biome in &creature.biomes {
            if !combined_creature.biomes.contains(biome) {
                combined_creature.biomes.push(biome.clone());
            }
        }

        // Loop over our tags and if they aren't in combined_creature, add them
        for tag in &creature.tags {
            if !combined_creature.tags.contains(tag) {
                combined_creature.tags.push(tag.clone());
            }
        }

        // If any of our other properties are not default, we need to apply them to the combined creature.
        if creature.frequency != 50 {
            combined_creature.frequency = creature.frequency;
        }
        if creature.population_number != [1, 1] {
            combined_creature.population_number = creature.population_number;
        }
        if creature.cluster_number != [1, 1] {
            combined_creature.cluster_number = creature.cluster_number;
        }
        if creature.underground_depth != [0, 0] {
            combined_creature.underground_depth = creature.underground_depth;
        }
        if !creature.general_baby_name.is_empty() {
            combined_creature.general_baby_name = creature.general_baby_name.clone();
        }
        if !creature.general_child_name.is_empty() {
            combined_creature.general_child_name = creature.general_child_name.clone();
        }
        if !creature.name.is_empty() {
            combined_creature.name = creature.name.clone();
        }
        if creature.tile.is_default() {
            combined_creature.tile = creature.tile.clone();
        }

        combined_creature
    }
}

#[typetag::serde]
impl RawObject for DFCreature {
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
        &ObjectType::Creature
    }
    fn parse_tag(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) {
            self.castes.last_mut().unwrap().parse_tag(key, value);
            return;
        }
        if !CREATURE_TOKENS.contains_key(key) {
            log::debug!("CreatureParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = CREATURE_TOKENS.get(key) else {
            log::warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed creature tag: {}",
                key
            );
            return;
        };

        match tag {
            CreatureTag::Biome => {
                self.biomes.push(String::from(value));
            }
            CreatureTag::Name => {
                self.name = Name::from_value(value);
            }
            CreatureTag::GeneralBabyName => {
                self.general_baby_name = SingPlurName::from_value(value);
            }
            CreatureTag::GeneralChildName => {
                self.general_child_name = SingPlurName::from_value(value);
            }
            CreatureTag::PrefString => {
                self.pref_strings.push(String::from(value));
            }
            CreatureTag::PopulationNumber => {
                self.population_number = parse_min_max_range(value).unwrap_or([1, 1]);
            }
            CreatureTag::Frequency => {
                self.frequency = value.parse::<u16>().unwrap_or(0);
            }
            CreatureTag::UndergroundDepth => {
                self.underground_depth = parse_min_max_range(value).unwrap_or([0, 0]);
            }
            CreatureTag::ClusterNumber => {
                self.cluster_number = parse_min_max_range(value).unwrap_or([1, 1]);
            }
            CreatureTag::CopyTagsFrom => {
                self.copy_tags_from = String::from(value);
            }
            CreatureTag::ApplyCreatureVariation => {
                self.apply_creature_variation.push(String::from(value));
            }
            CreatureTag::CreatureTile => {
                self.tile.set_character(value);
            }
            CreatureTag::AltTile => {
                self.tile.set_alt_character(value);
            }
            CreatureTag::Color => {
                self.tile.set_color(value);
            }
            CreatureTag::GlowColor => {
                self.tile.set_glow_color(value);
            }
            CreatureTag::GlowTile => {
                self.tile.set_glow_character(value);
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

#[typetag::serde]
impl CreatureVariationRequirements for DFCreature {
    fn remove_tag(&mut self, key: &str) {
        self.remove_tag_and_value(key, "");
    }

    fn remove_tag_and_value(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) {
            self.castes
                .last_mut()
                .unwrap()
                .remove_tag_and_value(key, value);
            return;
        }
        if !CREATURE_TOKENS.contains_key(key) {
            log::debug!("CreatureParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = CREATURE_TOKENS.get(key) else {
            log::warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed creature tag: {}",
                key
            );
            return;
        };

        match tag {
            CreatureTag::Biome => {
                self.biomes.retain(|x| x != value);
            }
            CreatureTag::Name => {
                self.name = Name::default();
            }
            CreatureTag::GeneralBabyName => {
                self.general_baby_name = SingPlurName::default();
            }
            CreatureTag::GeneralChildName => {
                self.general_child_name = SingPlurName::default();
            }
            CreatureTag::PrefString => {
                self.pref_strings.retain(|x| x != value);
            }
            CreatureTag::PopulationNumber => {
                self.population_number = [1, 1];
            }
            CreatureTag::Frequency => {
                self.frequency = 0;
            }
            CreatureTag::UndergroundDepth => {
                self.underground_depth = [0, 0];
            }
            CreatureTag::ClusterNumber => {
                self.cluster_number = [1, 1];
            }
            CreatureTag::CopyTagsFrom => {
                self.copy_tags_from = String::default();
            }
            CreatureTag::ApplyCreatureVariation => {
                self.apply_creature_variation.retain(|x| x != value);
            }
            CreatureTag::CreatureTile => {
                self.tile.set_character("");
            }
            CreatureTag::AltTile => {
                self.tile.set_alt_character("");
            }
            CreatureTag::Color => {
                self.tile.set_color("");
            }
            CreatureTag::GlowColor => {
                self.tile.set_glow_color("");
            }
            CreatureTag::GlowTile => {
                self.tile.set_glow_character("");
            }
            _ => {
                self.tags.retain(|x| x != tag);
            }
        }
    }

    fn remove_tag_for_caste(&mut self, key: &str, caste: &str) {
        self.select_caste(caste);
        self.remove_tag(key);
    }

    fn remove_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str) {
        self.select_caste(caste);
        self.remove_tag_and_value(key, value);
    }

    fn add_tag(&mut self, key: &str) {
        self.parse_tag(key, "");
    }

    fn add_tag_and_value(&mut self, key: &str, value: &str) {
        self.parse_tag(key, value);
    }

    fn add_tag_for_caste(&mut self, key: &str, caste: &str) {
        self.select_caste(caste);
        self.parse_tag(key, "");
    }

    fn add_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str) {
        self.select_caste(caste);
        self.parse_tag(key, value);
    }
}
