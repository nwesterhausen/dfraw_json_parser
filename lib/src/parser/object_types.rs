use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

<<<<<<<< HEAD:lib/src/parser/object_types.rs
pub static OBJECT_TOKEN_MAP: phf::Map<&'static str, ObjectType> = phf::phf_map! {
========
/// A PHF map of the string literal object tokens to the `ObjectType` enum.
pub static OBJECT_TOKENS: phf::Map<&'static str, ObjectType> = phf::phf_map! {
>>>>>>>> c2812957821240fff30b78553e73f23e904207e2:src/parser/object_type.rs
    "CREATURE" => ObjectType::Creature,
    "INORGANIC" => ObjectType::Inorganic,
    "PLANT" => ObjectType::Plant,
    "ITEM" => ObjectType::Item,
    "ITEM_AMMO" => ObjectType::ItemAmmo,
    "ITEM_ARMOR" => ObjectType::ItemArmor,
    "ITEM_FOOD" => ObjectType::ItemFood,
    "ITEM_GLOVES" => ObjectType::ItemGloves,
    "ITEM_HELM" => ObjectType::ItemHelm,
    "ITEM_INSTRUMENT" => ObjectType::ItemInstrument,
    "ITEM_PANTS" => ObjectType::ItemPants,
    "ITEM_SHIELD" => ObjectType::ItemShield,
    "ITEM_SHOES" => ObjectType::ItemShoes,
    "ITEM_SIEGEAMMO" => ObjectType::ItemSiegeAmmo,
    "ITEM_TOOL" => ObjectType::ItemTool,
    "ITEM_TOY" => ObjectType::ItemToy,
    "ITEM_TRAPCOMP" => ObjectType::ItemTrapComponent,
    "ITEM_WEAPON" => ObjectType::ItemWeapon,
    "BUILDING" => ObjectType::Building,
    "BUILDING_WORKSHOP" => ObjectType::BuildingWorkshop,
    "BUILDING_FURNACE" => ObjectType::BuildingFurnace,
    "REACTION" => ObjectType::Reaction,
    "GRAPHICS" => ObjectType::Graphics,
    "MATERIAL_TEMPLATE" => ObjectType::MaterialTemplate,
    "BODY_DETAIL_PLAN" => ObjectType::BodyDetailPlan,
    "BODY" => ObjectType::Body,
    "ENTITY" => ObjectType::Entity,
    "LANGUAGE" => ObjectType::Language,
    "TRANSLATION" => ObjectType::Translation,
    "TISSUE_TEMPLATE" => ObjectType::TissueTemplate,
    "CREATURE_VARIATION" => ObjectType::CreatureVariation,
    "TEXT_SET" => ObjectType::TextSet,
    "TILE_PAGE" => ObjectType::TilePage,
    "DESCRIPTOR_COLOR" => ObjectType::DescriptorColor,
    "DESCRIPTOR_PATTERN" => ObjectType::DescriptorPattern,
    "DESCRIPTOR_SHAPE" => ObjectType::DescriptorShape,
    "PALETTE" => ObjectType::Palette,
    "MUSIC" => ObjectType::Music,
    "SOUND" => ObjectType::Sound,
    "INTERACTION" => ObjectType::Interaction,
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// The various types of raw objects. These are identified in the raws by the `OBJECT` token.
///
/// e.g. `[OBJECT:CREATURE]`
pub enum ObjectType {
    Creature,
    Inorganic,
    Plant,
    Item,
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
    Building,
    BuildingWorkshop,
    BuildingFurnace,
    Reaction,
    Graphics,
    MaterialTemplate,
    BodyDetailPlan,
    Body,
    Entity,
    Language,
    Translation,
    TissueTemplate,
    CreatureVariation,
    TextSet,
    TilePage,
    DescriptorColor,
    DescriptorPattern,
    DescriptorShape,
    Palette,
    Music,
    Sound,
    Interaction,
    #[default]
    Unknown,
    SelectCreature,
    CreatureCaste,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
