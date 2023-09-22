use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

pub static OBJECT_TOKENS: phf::Map<&'static str, ObjectType> = phf::phf_map! {
    "CREATURE" => ObjectType::Creature,
    "INORGANIC" => ObjectType::Inorganic,
    "PLANT" => ObjectType::Plant,
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
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ObjectType {
    Creature,
    Inorganic,
    Plant,
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
    #[default]
    Unknown,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
