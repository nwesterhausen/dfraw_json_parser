use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum InorganicToken {
    /// Used on metals, causes the metal to be made into wafers instead of bars.
    Wafers,
    /// Causes the stone to form hollow tubes leading to the Underworld. Used for raw adamantine. When mined, stone has a 100% yield.
    /// If no material with this token exists, hollow veins will instead be made of the first available inorganic, usually iron. Implies \[SPECIAL\].
    DeepSpecial,
    /// Allows the ore to be smelted into metal in the smelter. Each token with a non-zero chance causes the game to roll d100 four times,
    /// each time creating one bar of the type requested on success.
    MetalOre,
    /// Allows strands to be extracted from the metal at a craftsdwarf's workshop.
    ThreadMetal,
    /// Causes the stone to line the landscape of the Underworld. Used for slade. When mined (if it's mineable), stone has a 100% yield. If no material with this token exists,
    /// other materials will be used in place of slade. Underworld spires will still be referred to as a "spire of slade" in the world's history.
    DeepSurface,
    /// Allows the stone to support an aquifer.
    Aquifer,
    /// Causes the material to form metamorphic layers.
    Metamorphic,
    /// Causes the material to form sedimentary layers.
    Sedimentary,
    /// Causes the material to form soil layers, allowing it to appear in (almost) any biome. Mining is faster and produces no stones.
    Soil,
    /// Causes the material to form pelagic sediment layers beneath deep oceans. Mining is faster and produces no stones.
    SoilOcean,
    /// Causes the material to form sand layers, allowing it to appear in sand deserts and shallow oceans. Mining is faster and produces no stones.
    /// Sand layers can also be used for making glass. Can be combined with \[SOIL\].
    SoilSand,
    /// Permits an already \[SEDIMENTARY\] stone layer to appear underneath shallow ocean regions.
    SedimentaryOceanShallow,
    /// Permits an already \[SEDIMENTARY\] stone layer to appear underneath deep ocean regions.
    SedimentaryOceanDeep,
    /// Causes the material to form igneous intrusive layers.
    IgneousExtrusive,
    /// Causes the material to form igneous extrusive layers.
    IgneousIntrusive,
    /// Specifies what types of layers will contain this mineral. Multiple instances of the same token segment will cause the rock type to occur more frequently,
    /// but won't increase its abundance in the specified environment. See below.
    Environment,
    /// Specifies which specific minerals will contain this mineral. See below.
    EnvironmentSpecific,
    /// Specifies that the stone is created when combining water and magma, also causing it to line the edges of magma pools and volcanoes.
    /// If multiple minerals are marked as lava stones, a different one will be used in each biome or geological region.
    Lava,
    /// Prevents the material from showing up in certain places. AI-controlled entities won't use the material to make items and don't bring it in caravans,
    /// though the player can use it as normal. Also, inorganic generated creatures (forgotten beasts, titans, demons) will never be composed of this material.
    /// Explicitly set by all evil weather materials and implied by \[DEEP_SURFACE\] and \[DEEP_SPECIAL\].
    Special,
    /// Indicates that this is a generated material. Cannot be specified in user-defined raws.
    Generated,
    /// Found on random-generated metals and cloth. Marks this material as usable by Deity-created generated entities.
    Divine,
    /// Found on divine materials. Presumably links the material to a god of the same sphere.
    Sphere,
    /// Default value means parsing error.
    #[default]
    Unknown,
}



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum EnvironmentClass {
    /// Will appear in every stone.
    AllStone,
    /// Will appear in all igneous layers
    IgneousAll,
    /// Will appear in igneous extrusive layers
    IgneousExtrusive,
    /// Will appear in igneous intrusive layers
    IgneousIntrusive,
    /// Will appear in soil.
    Soil,
    /// Will appear in sand.
    SoilSand,
    /// Will appear in soil in the oceans.
    SoilOcean,
    /// Will appear in sedimentary layers.
    Sedimentary,
    /// Will appear in metamorphic layers.
    Metamorphic,
    /// Will appear in alluvial layers.
    Alluvial,
    /// Default value means parsing error.
    #[default]
    None,
}



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum InclusionType {
    /// Large ovoids that occupy their entire 48x48 embark tile. Microcline is an example. When mined, stone has a 25% yield (as with layer stones).
    Cluster,
    /// Blobs of 3-9 tiles. Will always be successfully mined. Red pyropes are an example. When mined, stone has a 100% yield.
    ClusterSmall,
    /// Single tiles. Will always be successfully mined. Clear diamonds are an example. When mined, stone has a 100% yield.
    ClusterOne,
    /// Large streaks of stone. Native gold is an example. When mined, stone has a 33% yield instead of the usual 25%.
    Vein,
    /// Default value means parsing error.
    #[default]
    None,
}

impl EnvironmentClass {
    pub fn is_default(&self) -> bool {
        *self == EnvironmentClass::None
    }
}
impl InclusionType {
    pub fn is_default(&self) -> bool {
        *self == InclusionType::None
    }
}
impl std::fmt::Display for InclusionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InclusionType::Cluster => write!(f, "Cluster"),
            InclusionType::ClusterSmall => write!(f, "ClusterSmall"),
            InclusionType::ClusterOne => write!(f, "ClusterOne"),
            InclusionType::Vein => write!(f, "Vein"),
            InclusionType::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Display for EnvironmentClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EnvironmentClass::AllStone => write!(f, "AllStone"),
            EnvironmentClass::IgneousAll => write!(f, "IgneousAll"),
            EnvironmentClass::IgneousExtrusive => write!(f, "IgneousExtrusive"),
            EnvironmentClass::IgneousIntrusive => write!(f, "IgneousIntrusive"),
            EnvironmentClass::Soil => write!(f, "Soil"),
            EnvironmentClass::SoilSand => write!(f, "SoilSand"),
            EnvironmentClass::SoilOcean => write!(f, "SoilOcean"),
            EnvironmentClass::Sedimentary => write!(f, "Sedimentary"),
            EnvironmentClass::Metamorphic => write!(f, "Metamorphic"),
            EnvironmentClass::Alluvial => write!(f, "Alluvial"),
            EnvironmentClass::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Display for InorganicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InorganicToken::Wafers => write!(f, "Wafers"),
            InorganicToken::DeepSpecial => write!(f, "DeepSpecial"),
            InorganicToken::MetalOre => write!(f, "MetalOre"),
            InorganicToken::ThreadMetal => write!(f, "ThreadMetal"),
            InorganicToken::DeepSurface => write!(f, "DeepSurface"),
            InorganicToken::Aquifer => write!(f, "Aquifer"),
            InorganicToken::Metamorphic => write!(f, "Metamorphic"),
            InorganicToken::Sedimentary => write!(f, "Sedimentary"),
            InorganicToken::Soil => write!(f, "Soil"),
            InorganicToken::SoilOcean => write!(f, "SoilOcean"),
            InorganicToken::SoilSand => write!(f, "SoilSand"),
            InorganicToken::SedimentaryOceanShallow => write!(f, "SedimentaryOceanShallow"),
            InorganicToken::SedimentaryOceanDeep => write!(f, "SedimentaryOceanDeep"),
            InorganicToken::IgneousExtrusive => write!(f, "IgneousExtrusive"),
            InorganicToken::IgneousIntrusive => write!(f, "IgneousIntrusive"),
            InorganicToken::Environment => write!(f, "Environment"),
            InorganicToken::EnvironmentSpecific => write!(f, "EnvironmentSpecific"),
            InorganicToken::Lava => write!(f, "Lava"),
            InorganicToken::Special => write!(f, "Special"),
            InorganicToken::Generated => write!(f, "Generated"),
            InorganicToken::Divine => write!(f, "Divine"),
            InorganicToken::Sphere => write!(f, "Sphere"),
            InorganicToken::Unknown => write!(f, "Unknown"),
        }
    }
}
