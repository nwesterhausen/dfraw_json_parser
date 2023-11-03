mod phf_table;
mod raw;
mod tokens;

pub use phf_table::CREATURE_EFFECT_PROPERTY_TOKENS as PROPERTY_TOKENS;
pub use phf_table::CREATURE_EFFECT_TOKENS as EFFECT_TOKENS;
pub use raw::CreatureEffect;
pub use tokens::CreatureEffectProperty as PropertyTag;
pub use tokens::CreatureEffectToken as EffectTag;
