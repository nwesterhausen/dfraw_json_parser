use serde::{Deserialize, Serialize};

use super::refs::RAW_TOKEN_RE;

// The type of raw describes the type of object it is.
#[derive(Serialize, Deserialize, Clone)]
pub enum RawObjectKind {
    // Allowed Top-Level Raw Object Type (for entire file)
    Body,
    BodyDetailPlan,
    Building,
    Creature,
    CreatureVariation,
    DescriptorColor,
    DescriptorPattern,
    DescriptorShape,
    Entity,
    Inorganic,
    Graphics,
    Item,
    Language,
    MaterialTemplate,
    Music,
    Plant,
    Reaction,
    Sound,
    TextSet,
    TissueTemplate,
    None,
    // Allowed Sub-Level Raw Object Type (for individual objects, also allowed are the top-level types)
    // Building Sub-Types
    BuildingWorkshop,
    // Descriptor Sub-Types
    Color,
    ColorPattern,
    Shape,
    // Graphics Sub-Types
    TileGraphics,
    WoodGraphics,
    RoughGemGraphics,
    BoulderGraphics,
    BarsGraphics,
    // Item Sub-Types
    ItemAmmo,
    ItemArmor,
    ItemFood,
    ItemGloves,
    ItemHelm,
    ItemInstrument,
    ItemPants,
    ItemShield,
    ItemShoes,
    ItemSiegeAmmo,
    ItemTool,
    ItemToy,
    ItemTrapComponent,
    ItemWeapon,
    // Language Sub-Types
    Symbol,
    Translation,
    Word,
}

impl RawObjectKind {
    // Takes the string describing the object type and returns the enum variant
    // This works for both the top-level and sub-level types
    // Top-level ex: [OBJECT:CREATURE]
    // Sub-level ex: [CREATURE:COW]
    pub fn from_string(object_type: &str) -> Self {
        if RAW_TOKEN_RE.is_match(object_type) {
            // If we got the entire line, we need to extract the object type
            let caps = RAW_TOKEN_RE.captures(object_type).unwrap();
            // We can check if the 'key' is "OBJECT" and then we match the 'value'
            // If 'key' is not "OBJECT" then we match based on 'key'
            if caps.name("key").unwrap().as_str() == "OBJECT" {
                return RawObjectKind::from_string(caps.name("value").unwrap().as_str());
            }
            return RawObjectKind::from_string(caps.name("key").unwrap().as_str());
        }

        match object_type {
            "BODY" => RawObjectKind::Body,
            "BODY_DETAIL_PLAN" => RawObjectKind::BodyDetailPlan,
            "BUILDING" => RawObjectKind::Building,
            "CREATURE" => RawObjectKind::Creature,
            "CREATURE_VARIATION" => RawObjectKind::CreatureVariation,
            "DESCRIPTOR_COLOR" => RawObjectKind::DescriptorColor,
            "DESCRIPTOR_PATTERN" => RawObjectKind::DescriptorPattern,
            "DESCRIPTOR_SHAPE" => RawObjectKind::DescriptorShape,
            "ENTITY" => RawObjectKind::Entity,
            "INORGANIC" => RawObjectKind::Inorganic,
            "GRAPHICS" => RawObjectKind::Graphics,
            "ITEM" => RawObjectKind::Item,
            "LANGUAGE" => RawObjectKind::Language,
            "MATERIAL_TEMPLATE" => RawObjectKind::MaterialTemplate,
            "MUSIC" => RawObjectKind::Music,
            "PLANT" => RawObjectKind::Plant,
            "REACTION" => RawObjectKind::Reaction,
            "SOUND" => RawObjectKind::Sound,
            "TEXT_SET" => RawObjectKind::TextSet,
            "TISSUE_TEMPLATE" => RawObjectKind::TissueTemplate,
            // Sub-Types
            "BUILDING_WORKSHOP" => RawObjectKind::BuildingWorkshop,
            "COLOR" => RawObjectKind::Color,
            "COLOR_PATTERN" => RawObjectKind::ColorPattern,
            "SHAPE" => RawObjectKind::Shape,
            "TILE_GRAPHICS" => RawObjectKind::TileGraphics,
            "WOOD_GRAPHICS" => RawObjectKind::WoodGraphics,
            "ROUGH_GEM_GRAPHICS" => RawObjectKind::RoughGemGraphics,
            "BOULDER_GRAPHICS" => RawObjectKind::BoulderGraphics,
            "BARS_GRAPHICS" => RawObjectKind::BarsGraphics,
            "ITEM_AMMO" => RawObjectKind::ItemAmmo,
            "ITEM_ARMOR" => RawObjectKind::ItemArmor,
            "ITEM_FOOD" => RawObjectKind::ItemFood,
            "ITEM_GLOVES" => RawObjectKind::ItemGloves,
            "ITEM_HELM" => RawObjectKind::ItemHelm,
            "ITEM_INSTRUMENT" => RawObjectKind::ItemInstrument,
            "ITEM_PANTS" => RawObjectKind::ItemPants,
            "ITEM_SHIELD" => RawObjectKind::ItemShield,
            "ITEM_SHOES" => RawObjectKind::ItemShoes,
            "ITEM_SIEGEAMMO" => RawObjectKind::ItemSiegeAmmo,
            "ITEM_TOOL" => RawObjectKind::ItemTool,
            "ITEM_TOY" => RawObjectKind::ItemToy,
            "ITEM_TRAPCOMP" => RawObjectKind::ItemTrapComponent,
            "ITEM_WEAPON" => RawObjectKind::ItemWeapon,
            "SYMBOL" => RawObjectKind::Symbol,
            "TRANSLATION" => RawObjectKind::Translation,
            "WORD" => RawObjectKind::Word,
            _ => RawObjectKind::None,
        }
    }
}
