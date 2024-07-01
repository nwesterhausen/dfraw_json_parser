//! The enum variants are used to represent the raw tokens in a more Rust-friendly way.

pub mod biome;
pub mod caste;
pub mod color_modification;
pub mod condition;
pub mod creature;
pub mod creature_effect;
pub mod creature_effect_property;
pub mod creature_variation;
pub mod entity;
pub mod environment_class;
pub mod graphic_type;
pub mod growth;
pub mod inclusion_type;
pub mod inorganic;
pub mod plant_graphic_template;
pub mod tile_page;

pub use biome::BiomeTag;
pub use caste::CasteTag;
pub use color_modification::ColorModificationTag;
pub use condition::ConditionTag;
pub use creature::CreatureTag;
pub use creature_effect::CreatureEffectTag;
pub use creature_effect_property::CreatureEffectPropertyTag;
pub use creature_variation::CreatureVariationTag;
pub use entity::EntityTag;
pub use environment_class::EnvironmentClassTag;
pub use graphic_type::GraphicTypeTag;
pub use growth::GrowthTag;
pub use inclusion_type::InclusionTypeTag;
pub use inorganic::InorganicTag;
pub use plant_graphic_template::PlantGraphicTemplateTag;
pub use tile_page::TilePageTag;
