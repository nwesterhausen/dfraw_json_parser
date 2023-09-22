use serde::{Deserialize, Serialize};

use crate::parser::{
    body_size::DFBodySize,
    milkable::DFMilkable,
    names::{Name, SingPlurName},
    ranges::parse_min_max_range,
    ranges::Ranges,
    tile::DFTile,
};

use super::{phf_table::CASTE_TOKENS, tokens::CasteTag};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DFCaste {
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
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    clutch_size: [u16; 2],
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    litter_size: [u16; 2],
    #[serde(skip_serializing_if = "Ranges::min_max_is_zeroes")]
    max_age: [u16; 2],
    // Integer tokens
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    baby: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    child: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    difficulty: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    egg_size: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero_u8")]
    grass_trample: u8,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    grazer: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    low_light_vision: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    pet_value: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    pop_ratio: u32,
    #[serde(skip_serializing_if = "Ranges::is_zero")]
    change_body_size_percentage: u32,
    // Arrays
    #[serde(skip_serializing_if = "Vec::is_empty")]
    creature_class: Vec<String>,
    // Special Tokens
    #[serde(skip_serializing_if = "Vec::is_empty")]
    body_size: Vec<DFBodySize>,
    #[serde(skip_serializing_if = "DFMilkable::is_default")]
    milkable: DFMilkable,
    #[serde(skip_serializing_if = "DFTile::is_default")]
    tile: DFTile,
}

impl DFCaste {
    pub fn new(name: &str) -> DFCaste {
        DFCaste {
            identifier: String::from(name),
            ..DFCaste::default()
        }
    }
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = CASTE_TOKENS.get(key) else {
            log::warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        // If value is empty, add the tag to the last caste
        if value.is_empty() {
            self.tags.push(tag.clone());
            return;
        }

        match tag {
            CasteTag::Description => self.description = String::from(value),
            CasteTag::EggSize => self.egg_size = value.parse::<u32>().unwrap_or_default(),
            CasteTag::Baby => self.baby = value.parse::<u32>().unwrap_or_default(),
            CasteTag::Child => self.child = value.parse::<u32>().unwrap_or_default(),
            CasteTag::Difficulty => self.difficulty = value.parse::<u32>().unwrap_or_default(),
            CasteTag::Grazer => self.grazer = value.parse::<u32>().unwrap_or_default(),
            CasteTag::GrassTrample => self.grass_trample = value.parse::<u8>().unwrap_or_default(),
            CasteTag::LowLightVision => {
                self.low_light_vision = value.parse::<u32>().unwrap_or_default();
            }
            CasteTag::PopRatio => self.pop_ratio = value.parse::<u32>().unwrap_or_default(),
            CasteTag::PetValue => self.pet_value = value.parse::<u32>().unwrap_or_default(),
            CasteTag::ClutchSize => {
                self.clutch_size = parse_min_max_range(value).unwrap_or_default();
            }
            CasteTag::LitterSize => {
                self.litter_size = parse_min_max_range(value).unwrap_or_default();
            }
            CasteTag::MaxAge => self.max_age = parse_min_max_range(value).unwrap_or_default(),
            CasteTag::CreatureClass => self.creature_class.push(String::from(value)),
            CasteTag::BodySize => {
                self.body_size.push(DFBodySize::from_value(value));
            }
            CasteTag::Milkable => self.milkable = DFMilkable::from_value(value),
            CasteTag::BabyName => self.baby_name = SingPlurName::from_value(value),
            CasteTag::CasteName => self.caste_name = Name::from_value(value),
            CasteTag::ChildName => self.child_name = SingPlurName::from_value(value),
            CasteTag::CasteTile => self.tile.set_character(value),
            CasteTag::CasteAltTile => self.tile.set_alt_character(value),
            CasteTag::CasteColor => self.tile.set_color(value),
            CasteTag::CasteGlowTile => self.tile.set_glow_character(value),
            CasteTag::CasteGlowColor => self.tile.set_glow_color(value),
            CasteTag::ChangeBodySizePercent => {
                self.change_body_size_percentage = value.parse::<u32>().unwrap_or_default();
            }
            _ => self.tags.push(tag.clone()),
        }
    }

    pub(crate) fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
