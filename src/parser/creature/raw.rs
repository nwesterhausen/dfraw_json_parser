use serde::{Deserialize, Serialize};

use crate::parser::{
    biomes::BIOMES,
    creature_caste::{phf_table::CASTE_TOKENS, raw::Caste},
    creature_variation::raw::CreatureVariationRequirements,
    helpers::object_id::build_object_id_from_pieces,
    names::{Name, SingPlurName},
    object_types::ObjectType,
    ranges::parse_min_max_range,
    raws::{RawMetadata, RawObject},
    searchable::{clean_search_vec, Searchable},
    select_creature::raw::SelectCreature,
    serializer_helper,
    tile::Tile,
};

use super::{phf_table::CREATURE_TOKENS, tokens::CreatureTag};

/// The `Creature` struct represents a creature in a Dwarf Fortress, with the properties
/// that can be set in the raws. Not all the raws are represented here, only the ones that
/// are currently supported by the library.
///
/// Some items like `CREATURE_VARIATION` and `CREATURE_VARIATION_CASTE` are saved in their raw
/// format. `SELECT_CREATURE` is saved here as a sub-creature object with all the properties
/// from that raw. This is because the `SELECT_CREATURE` raws are used to create new creatures
/// based on the properties of the creature they are applied to. But right now the application
/// of those changes is not applied, in order to preserve the original creature. So instead,
/// they are saved and can be applied later (at the consumer's discretion).
#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Creature {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    identifier: String,
    castes: Vec<Caste>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<CreatureTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    biomes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pref_strings: Vec<String>,
    #[serde(skip_serializing_if = "Tile::is_default")]
    tile: Tile,
    // integers
    #[serde(skip_serializing_if = "serializer_helper::is_default_frequency")]
    frequency: u16, //Defaults to 50 if not specified
    // [min, max] ranges
    /// Default [1, 1]
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_ones")]
    cluster_number: [u16; 2],
    /// Default [1, 1]
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_ones")]
    population_number: [u16; 2],
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    select_creature_variation: Vec<SelectCreature>,
}

impl Creature {
    /// Returns a `Creature` object with default values.
    ///
    /// Returns:
    ///
    /// An empty instance of `Creature`.
    pub fn empty() -> Creature {
        Creature {
            castes: vec![Caste::new("ALL")],
            population_number: [1, 1],
            cluster_number: [1, 1],
            frequency: 50,
            ..Creature::default()
        }
    }

    /// Create a new instance of a `Creature` with the given identifier and metadata.
    ///
    /// Arguments:
    ///
    /// * `identifier`: A string that represents the identifier of the creature. It is used to uniquely
    /// identify the creature.
    /// * `metadata`: The `metadata` parameter is of type `RawMetadata` and is used to provide
    /// additional information about the raws the `Creature` is found in.
    ///
    /// Returns:
    ///
    /// a `Creature` object.
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Creature {
        Creature {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            frequency: 50,
            castes: vec![Caste::new("ALL")],
            population_number: [1, 1],
            cluster_number: [1, 1],
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Creature),
            ..Creature::default()
        }
    }

    /// The function `get_copy_tags_from` returns a reference to the `copy_tags_from` field.
    ///
    /// Returns:
    ///
    /// The private field `copy_tags_from`.
    pub fn get_copy_tags_from(&self) -> &str {
        &self.copy_tags_from
    }

    /// Adds a `SelectCreature` object to the internal `SelectCreature` vector.
    ///
    /// Arguments:
    ///
    /// * `select_creature`: The parameter `select_creature` is of type `SelectCreature`.
    pub fn push_select_creature_variation(&mut self, select_creature: SelectCreature) {
        self.select_creature_variation.push(select_creature);
    }

    /// Extends the internal `SelectCreature` vector with the elements from the `select_creature_vec`
    /// vector. This is a convenience function to enable bulk addition of `SelectCreature` objects.
    ///
    /// Arguments:
    ///
    /// * `select_creature_vec`: A vector of `SelectCreature` objects.
    pub fn extend_select_creature_variation(&mut self, select_creature_vec: Vec<SelectCreature>) {
        self.select_creature_variation.extend(select_creature_vec);
    }

    /// The function `add_caste` adds a new `Caste` object with the given name to a vector called
    /// `castes`.
    ///
    /// Arguments:
    ///
    /// * `name`: The `name` parameter is a string that represents the name of the caste to add.
    pub fn add_caste(&mut self, name: &str) {
        self.castes.push(Caste::new(name));
    }

    /// The function `select_caste` moves a caste to the end of a list if it matches the given name,
    /// otherwise it adds a new caste with the given name. This essentially allows the other functions
    /// to assume that the caste they are working with is the last one in the list.
    ///
    /// Arguments:
    ///
    /// * `name`: The `name` parameter is a string that represents the identifier of the caste to select.
    pub fn select_caste(&mut self, name: &str) {
        // Find the caste
        let mut index = 0;
        for (i, caste) in self.castes.iter().enumerate() {
            if caste.get_identifier().eq(name) {
                index = i;
                break;
            }
        }

        if index == 0 {
            // If we have no castes, add a new one
            if self.castes.is_empty() {
                return self.add_caste(name);
            } else if let Some(caste) = self.castes.get(index) {
                // (If we're here, we're at index 0 and the caste list is not empty)
                // If the caste doesn't match the one we need, add a new one
                if !caste.get_identifier().eq(name) {
                    return self.add_caste(name);
                }
            }
        }

        // Move the caste to the end of the list
        let caste = self.castes.remove(index);
        self.castes.push(caste);
    }

    /// Checks if a given name exists in the list of castes.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the `identifier` of the caste to check for.
    ///
    /// Returns:
    ///
    /// Returns true if there is a caste with the given name in this creature's caste list,
    /// and false otherwise.
    pub fn has_caste(&self, name: &str) -> bool {
        for caste in &self.castes {
            if caste.get_identifier().eq(name) {
                return true;
            }
        }
        false
    }

    /// Returns a vector of object IDs from the creature's `SelectCreature` vector. Essentially,
    /// it's the list of object IDs that have been added to this creature and then can be removed
    /// from the master raw list.
    ///
    /// Returns:
    ///
    /// Returns a vector of `object_id`s.
    pub fn get_child_object_ids(&self) -> Vec<&str> {
        let mut object_ids = Vec::new();
        for select_creature in &self.select_creature_variation {
            object_ids.push(select_creature.get_object_id());
        }
        object_ids
    }

    /// Takes two `Creature` objects and creates a new `Creature` object
    /// by combining their tags and properties.
    ///
    /// Arguments:
    ///
    /// * `creature`: A reference to the creature that will receive the copied tags.
    /// * `creature_to_copy_from`: A reference to the Creature object from which we want to copy the
    /// tags.
    ///
    /// Returns:
    ///
    /// A combined `Creature`, which is a combination of the original creature and the
    /// creature to copy from.
    pub fn copy_tags_from(creature: &Creature, creature_to_copy_from: &Creature) -> Self {
        // Because anything specified in our self will override the copied tags, first we need to clone the creature
        let mut combined_creature = creature_to_copy_from.clone();
        // Now apply any tags that exist for us but not for the one we copy.
        // So we need to go through all our properties and castes and overwrite what exists on the combined creature.

        // our metadata is preserved
        combined_creature.metadata = creature.metadata.clone();
        // our identifier is preserved
        combined_creature.identifier = creature.identifier.clone();
        // our object_id is preserved
        combined_creature.object_id = creature.object_id.clone();

        // We need to loop over our castes and apply any differences.
        for caste in &creature.castes {
            let caste_identifier = caste.get_identifier();
            // If the caste exists in the combined creature, we need to apply the differences
            if combined_creature.has_caste(caste_identifier) {
                combined_creature.select_caste(caste_identifier);
                if let Some(combined_caste) = combined_creature.castes.last_mut() {
                    combined_caste.overwrite_caste(caste);
                }
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
        if !serializer_helper::is_default_frequency(&creature.frequency) {
            combined_creature.frequency = creature.frequency;
        }
        if !serializer_helper::min_max_is_ones(&creature.cluster_number) {
            combined_creature.population_number = creature.population_number;
        }
        if !serializer_helper::min_max_is_ones(&creature.cluster_number) {
            combined_creature.cluster_number = creature.cluster_number;
        }
        if !serializer_helper::min_max_is_zeroes(&creature.underground_depth) {
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
impl RawObject for Creature {
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
            log::trace!("CreatureParsing: Unknown tag {} with value {}", key, value);
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
        self.object_id.as_str()
    }
}

#[typetag::serde]
impl CreatureVariationRequirements for Creature {
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

impl Searchable for Creature {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        // Add caste search strings
        for caste in &self.castes {
            vec.extend(caste.get_search_vec());
        }
        // Add tags
        for tag in &self.tags {
            vec.push(tag.to_string());
        }
        // Add biomes
        for biome in &self.biomes {
            if let Some(biome_str) = BIOMES.get(biome) {
                vec.push((*biome_str).to_string());
            }
        }
        // Add pref strings
        vec.extend(self.pref_strings.clone());
        // Add name
        vec.extend(self.name.as_vec());
        // Add general baby name
        vec.extend(self.general_baby_name.as_vec());
        // Add general child name
        vec.extend(self.general_child_name.as_vec());
        // Add identifier
        vec.push(self.identifier.clone());

        clean_search_vec(vec.as_slice())
    }
}
