mod phf_table;
mod raw;
mod simple;
mod tokens;

pub use phf_table::CV_TOKENS as TOKEN_MAP;
pub use raw::CreatureVariation;
pub use raw::CreatureVariationRequirements as Requirements;
pub use tokens::CVTag as Token;
