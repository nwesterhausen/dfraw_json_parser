/// A material state that can be set in a material definition.
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type,
)]
pub enum MaterialStateTag {
    /// Solid state of the material
    Solid,
    /// Liquid state of the material
    Liquid,
    /// Gas state of the material
    Gas,
    /// `POWDER` or `SOLID_POWDER`
    Powder,
    /// `PASTE` or `SOLID_PASTE`
    Paste,
    /// `PRESSED` or `SOLID_PRESSED`
    Pressed,
    /// Default value is invalid, so its a hint that this is not set.
    #[default]
    Unknown,
    /// Denotes all possible material states
    All,
    /// Denotes '`Solid`', '`Powder`', '`Paste`', and '`Pressed`'
    AllSolid,
}
