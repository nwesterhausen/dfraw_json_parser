use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// A map of the object tokens to their respective object types.
pub static OBJECT_TOKEN_MAP: phf::Map<&'static str, ObjectType> = phf::phf_map! {
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

/// The various types of objects that are within the raw files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Hash, specta::Type)]
pub enum ObjectType {
    /// A creature
    Creature,
    /// An inorganic material
    Inorganic,
    /// A plant
    Plant,
    /// An item
    Item,
    /// An item of type ammo
    ItemAmmo,
    /// An item of type armor
    ItemArmor,
    /// An item of type food
    ItemFood,
    /// An item of type gloves
    ItemGloves,
    /// An item of type helm
    ItemHelm,
    /// An item of type instrument
    ItemInstrument,
    /// An item of type pants
    ItemPants,
    /// An item of type shield
    ItemShield,
    /// An item of type shoes
    ItemShoes,
    /// An item of type siege ammo
    ItemSiegeAmmo,
    /// An item of type tool
    ItemTool,
    /// An item of type toy
    ItemToy,
    /// An item of type trap component
    ItemTrapComponent,
    /// An item of type weapon
    ItemWeapon,
    /// A building
    Building,
    /// A workshop building
    BuildingWorkshop,
    /// A furnace building
    BuildingFurnace,
    /// A reaction
    Reaction,
    /// Graphics
    Graphics,
    /// A material template
    MaterialTemplate,
    /// A body detail plan
    BodyDetailPlan,
    /// A body
    Body,
    /// An entity
    Entity,
    /// A language
    Language,
    /// A translation
    Translation,
    /// A tissue template
    TissueTemplate,
    /// A creature variation
    CreatureVariation,
    /// A text set
    TextSet,
    /// A tile page
    TilePage,
    /// A descriptor color
    DescriptorColor,
    /// A descriptor pattern
    DescriptorPattern,
    /// A descriptor shape
    DescriptorShape,
    /// A palette
    Palette,
    /// Music
    Music,
    /// Sound
    Sound,
    /// An interaction
    Interaction,
    /// An unknown object type
    #[default]
    Unknown,
    /// `SelectCreature` tag
    SelectCreature,
    /// A creature caste
    CreatureCaste,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Creature => write!(f, "Creature"),
            ObjectType::Inorganic => write!(f, "Inorganic"),
            ObjectType::Plant => write!(f, "Plant"),
            ObjectType::Item => write!(f, "Item"),
            ObjectType::ItemAmmo => write!(f, "Ammo (Item)"),
            ObjectType::ItemArmor => write!(f, "Armor (Item)"),
            ObjectType::ItemFood => write!(f, "Food (Item)"),
            ObjectType::ItemGloves => write!(f, "Gloves (Item)"),
            ObjectType::ItemHelm => write!(f, "Helm (Item)"),
            ObjectType::ItemInstrument => write!(f, "Instrument (Item)"),
            ObjectType::ItemPants => write!(f, "Pants (Item)"),
            ObjectType::ItemShield => write!(f, "Shield (Item)"),
            ObjectType::ItemShoes => write!(f, "Shoes (Item)"),
            ObjectType::ItemSiegeAmmo => write!(f, "Siege Ammo (Item)"),
            ObjectType::ItemTool => write!(f, "Tool (Item)"),
            ObjectType::ItemToy => write!(f, "Toy (Item)"),
            ObjectType::ItemTrapComponent => write!(f, "Trap Component (Item)"),
            ObjectType::ItemWeapon => write!(f, "Weapon (Item)"),
            ObjectType::Building => write!(f, "Building"),
            ObjectType::BuildingWorkshop => write!(f, "Workshop Building"),
            ObjectType::BuildingFurnace => write!(f, "Furnace Building"),
            ObjectType::Reaction => write!(f, "Reaction"),
            ObjectType::Graphics => write!(f, "Graphics"),
            ObjectType::MaterialTemplate => write!(f, "Material Template"),
            ObjectType::BodyDetailPlan => write!(f, "Body Detail Plan"),
            ObjectType::Body => write!(f, "Body"),
            ObjectType::Entity => write!(f, "Entity"),
            ObjectType::Language => write!(f, "Language"),
            ObjectType::Translation => write!(f, "Translation"),
            ObjectType::TissueTemplate => write!(f, "Tissue Template"),
            ObjectType::CreatureVariation => write!(f, "Creature Variation"),
            ObjectType::TextSet => write!(f, "Text Set"),
            ObjectType::TilePage => write!(f, "Tile Page"),
            ObjectType::DescriptorColor => write!(f, "Color Descriptor"),
            ObjectType::DescriptorPattern => write!(f, "Pattern Descriptor"),
            ObjectType::DescriptorShape => write!(f, "Shape Descriptor"),
            ObjectType::Palette => write!(f, "Palette"),
            ObjectType::Music => write!(f, "Music"),
            ObjectType::Sound => write!(f, "Sound"),
            ObjectType::Interaction => write!(f, "Interaction"),
            ObjectType::Unknown => write!(f, "Unknown"),
            ObjectType::SelectCreature => write!(f, "Select Creature"),
            ObjectType::CreatureCaste => write!(f, "Creature Caste"),
        }
    }
}
