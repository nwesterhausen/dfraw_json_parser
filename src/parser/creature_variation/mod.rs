mod phf_table;
mod raw;
mod simple;
mod tokens;

pub use phf_table::CV_TOKENS;
pub use raw::CreatureVariation;
#[allow(clippy::module_name_repetitions)]
pub use raw::CreatureVariationRequirements;
pub use tokens::CVTag;
