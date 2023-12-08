mod phf_table;
mod raw;
mod tokens;

pub use phf_table::CREATURE_TOKENS as TOKEN_MAP;
pub use raw::Creature;
pub use tokens::CreatureTag as Token;
