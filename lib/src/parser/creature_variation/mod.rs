mod phf_table;
mod raw;
mod rules;
mod tokens;
mod traits;

pub use phf_table::CV_TOKENS as TOKEN_MAP;
pub use raw::CreatureVariation;
pub use rules::CreatureVariationRule as Rule;
pub use tokens::CVTag as Token;
pub use traits::CreatureVariationRequirements as Requirements;
