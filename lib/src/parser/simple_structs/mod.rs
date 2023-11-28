mod body_size;
mod color;
mod milkable;
mod temperatures;
mod tile;

/// Represents the size of a creature at a given age.
pub use body_size::BodySize;
/// Represents a color in DF Raws
pub use color::Color;
/// Represents a creature's milkable properties (i.e. what kind of milk and how often it can be milked)
pub use milkable::Milkable;
/// Represents the temperature of a material
pub use temperatures::Temperatures;
/// Represents a tile in DF Raws
pub use tile::Tile;
