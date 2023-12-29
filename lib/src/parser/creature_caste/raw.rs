use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{
    metadata::{RawObjectToken, TokenComplexity},
    names::{Name, SingPlurName},
    serializer_helper, BodySize, Milkable, Searchable, Tile,
};

use super::{phf_table::CASTE_TOKENS, tokens::CasteTag, Gait};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Caste {
    identifier: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<CasteTag>,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    // String Tokens
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    baby_name: SingPlurName,
    #[serde(skip_serializing_if = "Name::is_empty")]
    caste_name: Name,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    child_name: SingPlurName,
    // [min, max] ranges
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    clutch_size: [u32; 2],
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    litter_size: [u32; 2],
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    max_age: [u32; 2],
    // Integer tokens
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    baby: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    child: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    difficulty: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    egg_size: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    grass_trample: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    grazer: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    low_light_vision: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    pet_value: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    pop_ratio: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    change_body_size_percentage: u32,
    // Arrays
    #[serde(skip_serializing_if = "Vec::is_empty")]
    creature_class: Vec<String>,
    // Special Tokens
    #[serde(skip_serializing_if = "Vec::is_empty")]
    body_size: Vec<BodySize>,
    #[serde(skip_serializing_if = "Milkable::is_default")]
    milkable: Milkable,
    #[serde(skip_serializing_if = "Tile::is_default")]
    tile: Tile,
    /// The gaits by which the creature can move.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    gaits: Vec<Gait>,
}

impl Caste {
    pub fn new(name: &str) -> Caste {
        Caste {
            identifier: String::from(name),
            ..Caste::default()
        }
    }
    pub fn get_tags(&self) -> Vec<CasteTag> {
        self.tags.clone()
    }
    pub fn get_milkable(&self) -> Milkable {
        self.milkable.clone()
    }
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = CASTE_TOKENS.get(key) else {
            warn!(
                "CasteParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        // If value is empty, add the tag to the last caste
        if value.is_empty() {
            self.tags.push(tag.clone());
            return;
        }
        if let TokenComplexity::None = tag.get_complexity() {
            // If the tag is a TokenComplexity::None, then the value should be empty
            // So we should log the extra value before adding the tag to the last caste
            warn!(
                "Caste::parse_tag: tag {} has a value of {} but is a TokenComplexity::None as {:?}",
                key, value, tag
            );
            self.tags.push(tag.clone());
            return;
        }
        // Both simple and complex tags should have a value, and that needs to be parsed. So let the tag handle it.
        let Some(tag_and_value) = CasteTag::parse_token(key, value) else {
            warn!(
                "Caste::parse_tag: Called unwrap on a None value for tag {} with value {}",
                key, value
            );
            return;
        };
        self.tags.push(tag_and_value.clone());

        match tag_and_value {
            CasteTag::Description { description } => self.description = description.clone(),
            CasteTag::EggSize { size } => self.egg_size = size,
            CasteTag::Baby { age } => self.baby = age,
            CasteTag::Child { age } => self.child = age,
            CasteTag::Difficulty { difficulty } => self.difficulty = difficulty,
            CasteTag::Grazer { grazer } => self.grazer = grazer,
            CasteTag::GrassTrample { trample } => self.grass_trample = trample,
            CasteTag::LowLightVision { vision } => self.low_light_vision = vision,
            CasteTag::PopulationRatio { pop_ratio } => self.pop_ratio = pop_ratio,
            CasteTag::PetValue { pet_value } => self.pet_value = pet_value,
            CasteTag::ClutchSize { min, max } => self.clutch_size = [min, max],
            CasteTag::LitterSize { min, max } => self.litter_size = [min, max],
            CasteTag::MaxAge { min, max } => self.max_age = [min, max],
            CasteTag::CreatureClass { class } => {
                self.creature_class.push(class.clone());
            }
            CasteTag::BodySize { .. } => {
                self.body_size.push(BodySize::from_value(value));
            }
            CasteTag::Milkable { .. } => self.milkable = Milkable::from_value(value),
            CasteTag::BabyName { .. } => self.baby_name = SingPlurName::from_value(value),
            CasteTag::Name { .. } => self.caste_name = Name::from_value(value),
            CasteTag::ChildName { .. } => self.child_name = SingPlurName::from_value(value),
            CasteTag::Tile { .. } => self.tile.set_character(value),
            CasteTag::AltTile { .. } => self.tile.set_alt_character(value),
            CasteTag::Color { .. } => self.tile.set_color(value),
            CasteTag::GlowTile { .. } => self.tile.set_glow_character(value),
            CasteTag::GlowColor { .. } => self.tile.set_glow_color(value),
            CasteTag::ChangeBodySizePercent { .. } => {
                self.change_body_size_percentage = value.parse::<u32>().unwrap_or_default();
            }
            CasteTag::Gait { .. } => {
                self.gaits.push(Gait::from_value(value));
            }
            _ => {}
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn remove_tag_and_value(&mut self, key: &str, value: &str) {
        let Some(tag) = CASTE_TOKENS.get(key) else {
            warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        if let TokenComplexity::None = tag.get_complexity() {
            // If the tag is a TokenComplexity::None, then the value should be empty
            // So we should log the extra value before adding the tag to the last caste
            warn!(
                "Caste::remove_tag_and_value: tag {} has a value of {} but is a TokenComplexity::None as {:?}",
                key, value, tag
            );
            return;
        }

        // For simple and complex, some of the tags are stored as part of the struct.
        // So we need to remove them from the struct as well as the tags vector.
        let Some(tag_and_value) = CasteTag::parse_token(key, value) else {
            warn!(
                        "Caste::remove_tag_and_value: Called unwrap on a None value for tag {} with value {}",
                        key, value
                    );
            return;
        };
        self.tags.retain(|tag| tag != &tag_and_value);

        match tag_and_value {
            CasteTag::Description { .. } => self.description = String::new(),
            CasteTag::EggSize { .. } => self.egg_size = 0,
            CasteTag::Baby { .. } => self.baby = 0,
            CasteTag::Child { .. } => self.child = 0,
            CasteTag::Difficulty { .. } => self.difficulty = 0,
            CasteTag::Grazer { .. } => self.grazer = 0,
            CasteTag::GrassTrample { .. } => self.grass_trample = 0,
            CasteTag::LowLightVision { .. } => self.low_light_vision = 0,
            CasteTag::PopulationRatio { .. } => self.pop_ratio = 0,
            CasteTag::PetValue { .. } => self.pet_value = 0,
            CasteTag::ClutchSize { .. } => self.clutch_size = [0, 0],
            CasteTag::LitterSize { .. } => self.litter_size = [0, 0],
            CasteTag::MaxAge { .. } => self.max_age = [0, 0],
            CasteTag::CreatureClass { .. } => {
                // Remove the specific creature class from the creature classes vector
                self.creature_class.retain(|class| class != value);
            }
            CasteTag::BodySize { .. } => {
                // Remove the specific body size from the body sizes vector
                self.body_size
                    .retain(|body_size| body_size != &BodySize::from_value(value));
            }
            CasteTag::Milkable { .. } => self.milkable = Milkable::default(),
            CasteTag::BabyName { .. } => self.baby_name = SingPlurName::default(),
            CasteTag::Name { .. } => self.caste_name = Name::default(),
            CasteTag::ChildName { .. } => self.child_name = SingPlurName::default(),
            CasteTag::Tile { .. } | //=> self.tile = Tile::default(),
            CasteTag::AltTile { .. } | //=> self.tile = Tile::default(),
            CasteTag::Color { .. } | //=> self.tile = Tile::default(),
            CasteTag::GlowTile { .. } | //=> self.tile = Tile::default(),
            CasteTag::GlowColor { .. } => self.tile = Tile::default(),
            CasteTag::ChangeBodySizePercent { .. } => {
                self.change_body_size_percentage = 0;
            }
            CasteTag::Gait { .. } => {
                // Remove the specific gait from the gaits vector
                self.gaits.retain(|gait| gait != &Gait::from_value(value));
            }
            _ => {}
        }
    }

    pub fn overwrite_caste(&mut self, other: &Caste) {
        // Include any tags from other that aren't in self
        for tag in &other.tags {
            if !self.tags.contains(tag) {
                self.tags.push(tag.clone());
            }
        }
        // For any of the other's values that are not default, overwrite self's values
        if !other.description.is_empty() {
            self.description = other.description.clone();
        }
        if !other.baby_name.is_empty() {
            self.baby_name = other.baby_name.clone();
        }
        if !other.caste_name.is_empty() {
            self.caste_name = other.caste_name.clone();
        }
        if !other.child_name.is_empty() {
            self.child_name = other.child_name.clone();
        }
        if other.clutch_size != [0, 0] {
            self.clutch_size = other.clutch_size;
        }
        if other.litter_size != [0, 0] {
            self.litter_size = other.litter_size;
        }
        if other.max_age != [0, 0] {
            self.max_age = other.max_age;
        }
        if other.baby != 0 {
            self.baby = other.baby;
        }
        if other.child != 0 {
            self.child = other.child;
        }
        if other.difficulty != 0 {
            self.difficulty = other.difficulty;
        }
        if other.egg_size != 0 {
            self.egg_size = other.egg_size;
        }
        if other.grass_trample != 0 {
            self.grass_trample = other.grass_trample;
        }
        if other.grazer != 0 {
            self.grazer = other.grazer;
        }
        if other.low_light_vision != 0 {
            self.low_light_vision = other.low_light_vision;
        }
        if other.pet_value != 0 {
            self.pet_value = other.pet_value;
        }
        if other.pop_ratio != 0 {
            self.pop_ratio = other.pop_ratio;
        }
        if other.change_body_size_percentage != 0 {
            self.change_body_size_percentage = other.change_body_size_percentage;
        }
        if !other.creature_class.is_empty() {
            self.creature_class = other.creature_class.clone();
        }
        if !other.body_size.is_empty() {
            self.body_size = other.body_size.clone();
        }
        if !other.milkable.is_default() {
            self.milkable = other.milkable.clone();
        }
        if !other.tile.is_default() {
            self.tile = other.tile.clone();
        }
    }

    pub fn is_egg_layer(&self) -> bool {
        self.tags.contains(&CasteTag::LaysEggs)
    }
    pub fn is_milkable(&self) -> bool {
        self.has_tag(&CasteTag::Milkable {
            material: String::new(),
            frequency: 0,
        })
    }
    /// Returns true if the caste has the given tag, no values are checked.
    ///
    /// ## Arguments
    ///
    /// * `tag` - The tag to check for (note that any values are ignored)
    ///
    /// ## Returns
    ///
    /// True if the caste has the given tag, false otherwise.
    pub fn has_tag(&self, tag: &CasteTag) -> bool {
        for t in &self.tags {
            if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                return true;
            }
        }
        false
    }
}

impl Searchable for Caste {
    // Used to help extend things that own this caste
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Identifier
        vec.push(self.identifier.clone());
        // Name (and child/baby names)
        vec.extend(self.caste_name.as_vec());
        vec.extend(self.child_name.as_vec());
        vec.extend(self.baby_name.as_vec());
        // Creature Class
        vec.extend(self.creature_class.clone());
        // Description
        vec.push(self.description.clone());
        // If egg layer, include egg information
        if self.is_egg_layer() {
            vec.push(String::from("eggs"));
            vec.push(format!("{}", self.egg_size));
        }
        // If milkable, include milk information
        if self.is_milkable() {
            vec.push(String::from("milk"));
            vec.extend(self.milkable.as_vec());
        }
        // If flier, include flyer information
        if self.tags.contains(&CasteTag::Flier) {
            vec.push(String::from("flying flies flier"));
        }
        // If playable/civilized, include playable information
        if self.tags.contains(&CasteTag::OutsiderControllable) {
            vec.push(String::from("playable civilized"));
        }
        // Include difficulty if not 0
        if self.difficulty > 0 {
            vec.push(format!("{}", self.difficulty));
        }
        // Include pet value if not 0
        if self.pet_value > 0 {
            vec.push(format!("{}", self.pet_value));
        }
        // If speaks, include language information
        // If learns, include learn
        // If both, include "intelligent"
        if self.tags.contains(&CasteTag::Intelligent) || self.tags.contains(&CasteTag::CanSpeak) {
            vec.push(String::from("speaks language"));
        }
        if self.tags.contains(&CasteTag::Intelligent) || self.tags.contains(&CasteTag::CanLearn) {
            vec.push(String::from("learns"));
        }
        if self.tags.contains(&CasteTag::Intelligent)
            || (self.tags.contains(&CasteTag::CanSpeak) && self.tags.contains(&CasteTag::CanLearn))
        {
            vec.push(String::from("intelligent"));
        }

        vec
    }
}
