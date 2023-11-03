/// Contains the perfect hash table for biome tokens and the Biome enum.
mod phf_map;

/// Contains the Biome enum.
mod tokens;

pub use phf_map::BIOME_TOKENS;
pub use tokens::Biome;
