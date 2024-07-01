//! The enum variants are used to represent the raw tokens in a more Rust-friendly way.

pub mod biome;
pub mod caste;
pub mod creature;
pub mod creature_effect;

pub use biome::BiomeTag;
pub use caste::CasteTag;
pub use creature::CreatureTag;
pub use creature_effect::CreatureEffectPropertyTag;
pub use creature_effect::CreatureEffectTag;
