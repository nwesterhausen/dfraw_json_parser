use serde::{Deserialize, Serialize};

use crate::parser::{
    names::{Name, SingPlurName},
    object_types::ObjectType,
    ranges::parse_min_max_range,
    ranges::Ranges,
    raws::{RawMetadata, RawObject},
    tile::DFTile,
};

use super::{
    caste::DFCaste,
    phf_table::{CASTE_TOKENS, CREATURE_TOKENS},
    tokens::CreatureTag,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DFCreature {
    #[serde(skip_serializing_if = "RawMetadata::is_hidden")]
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
    #[serde(skip_serializing_if = "DFCreature::is_default_frequency")]
    frequency: u16, //Defaults to 50 if not specified
    // [min, max] ranges
    #[serde(skip_serializing_if = "Ranges::min_max_is_ones")]
    cluster_number: [u16; 2], //Defaults to 1:1 if not specified.
    #[serde(skip_serializing_if = "Ranges::min_max_is_ones")]
    population_number: [u16; 2], //default 1:1
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    underground_depth: [u16; 2], //default 0:0 (aboveground)
    // strings
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    general_baby_name: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    general_child_name: SingPlurName,
    #[serde(skip_serializing_if = "Name::is_empty")]
    name: Name,
    // Special tokens
    #[serde(skip_serializing_if = "String::is_empty")]
    copy_tags_from: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    apply_creature_variation: Vec<String>,
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
    pub fn new(identifier: &str, metadata: RawMetadata) -> DFCreature {
        DFCreature {
            identifier: String::from(identifier),
            metadata,
            frequency: 50,
            castes: vec![DFCaste::new("ALL")],
            population_number: [1, 1],
            cluster_number: [1, 1],
            ..DFCreature::default()
        }
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
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_default_frequency(frequency: &u16) -> bool {
        *frequency == 50
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
                "CreatureParsing: called `Option::unwrap()` on a `None` value for prsmed creature tag: {}",
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
}
