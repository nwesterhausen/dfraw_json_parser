mod phf_table;
mod raw;
mod tokens;

pub use phf_table::CREATURE_EFFECT_PROPERTY_TOKENS as PROPERTY_TOKEN_MAP;
pub use phf_table::CREATURE_EFFECT_TOKENS as TOKEN_MAP;
pub use raw::CreatureEffect;
pub use tokens::CreatureEffectProperty as PropertyToken;
pub use tokens::CreatureEffectToken as Token;
