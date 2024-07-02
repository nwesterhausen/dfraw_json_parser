//! Gait type tag definition.

/// An enum representing a gait type.
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq, specta::Type,
)]
pub enum GaitTypeTag {
    /// Travel on foot/the ground
    /// Used for moving normally over ground tiles.
    Walk,
    /// Travel on foot/the ground
    /// Used for moving over ground tiles whilst prone.
    Crawl,
    /// Climbing on walls, etc.
    /// Used for moving whilst climbing.
    Climb,
    /// Swimming in water/liquid
    /// Used for moving through tiles containing water or magma at a depth of at least 4/7.
    Swim,
    /// Flying through the air
    /// Used for moving through open space.
    Fly,
    /// Other gait type which is unexpected, but we can still handle it
    Other(String),
    #[default]
    /// Unknown gait type (unset)
    Unknown,
}
