/// The growth tag of a plant
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy,
)]
pub enum PlantGrowthTag {
    /// The beginning of a growth tag
    Growth,
    /// The name of the growth
    GrowthName,
    /// The item from the growth
    GrowthItem,
    /// The host tile the growth grows on
    GrowthHostTile,
    /// The trunk height percent of the growth
    GrowthTrunkHeightPercent,
    /// The growth density
    GrowthDensity,
    /// The growth timing
    GrowthTiming,
    /// The growth print
    GrowthPrint,
    /// If the growth has a seed
    GrowthHasSeed,
    /// If the growth drops off the plant
    GrowthDropsOff,
    /// If the growth drops off the plant and there is no cloud
    GrowthDropsOffNoCloud,
    /// An unknown growth tag
    #[default]
    Unknown,
}
