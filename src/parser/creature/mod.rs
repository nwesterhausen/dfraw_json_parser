/// Contains the perfect hash table for creature tokens and `CreatureTag` enum.
mod phf_table;
/// Contains the `Creature` raw struct and implementations.
mod raw;
/// Contains the `CreatureTag` enum.
mod tokens;

pub use phf_table::CREATURE_TOKENS;
pub use raw::Creature;
#[allow(clippy::module_name_repetitions)]
pub use tokens::CreatureTag;
