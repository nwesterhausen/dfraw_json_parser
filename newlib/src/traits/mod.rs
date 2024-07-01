//! Shared traits amongst various parsed objects.

pub mod creature_variation_requirements;
pub mod raw_object;
pub mod raw_object_token;
pub mod raw_object_token_to_any;
pub mod searchable;

pub use creature_variation_requirements::CreatureVariationRequirements;
pub use raw_object::RawObject;
pub use raw_object_token::RawObjectToken;
pub use raw_object_token_to_any::RawObjectTokenToAny;
pub use searchable::Searchable;
