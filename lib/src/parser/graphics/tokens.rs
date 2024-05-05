use serde::{Deserialize, Serialize};
use tracing::warn;

use super::phf_table::CONDITION_TAGS;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]

pub enum Condition {
    None,
    Condition,
    #[default]
    Default,
    Animated,
    Corpse,
    Child,
    Baby,
    TrainedHunter,
    TrainedWar,
    ListIcon,
    Skeleton,
    SkeletonWithSkull,
    Zombie,
    Necromancer,
    Male,
    Female,
    VampireCursed,
    Ghoul,
    DisturbedDead,
    Remains,
    Vermin,
    LightVermin,
    Hive,
    SwarmSmall,
    SwarmMedium,
    SwarmLarge,

    NotArtifact,
    CraftedArtifact,
    Dye,
    NotDyed,

    Crop,
    Seed,
    Picked,
    Shrub,
    Sapling,
    CropSprout,
    CropL,
    CropM,
    CropR,
    ShrubDead,

    NotChild,
    HaulCountMin,
    HaulCountMax,
    ItemWorn,
    ProfessionCategory,
    Class,
    SyndromeClass,
    Caste,
    TissueLayer,
    MaterialFlag,
    MaterialType,
    ShutOffIfItemPresent,
    RandomPartIndex,
    Ghost,
    TissueMayHaveColor,
    TissueMinLength,
    TissueMaxLength,
    TissueMinCurly,
    TissueMaxCurly,
    TissueMayHaveShaping,
    TissueNotShaped,
    TissueSwap,
    // Additional Conditions
    Layer,
    LayerSet,
    LayerGroup,
    EndLayerGroup,
    BodyUpper,
    CopyOfTemplate,

    // Professions (somewhat of a hack.. but some mods don't use profession category and instead call direct)
    Hammerman,
    MasterHammerman,
    Spearman,
    MasterSpearman,
    Wrestler,
    MasterWrestler,
    Axeman,
    MasterAxeman,
    Swordsman,
    MasterSwordsman,
    Maceman,
    MasterMaceman,
    Pikeman,
    MasterPikeman,
    Recruit,
    Thief,
    MasterThief,
    Lasher,
    MasterLasher,
    MonsterSlayer,
    Crossbowman,
    MasterCrossbowman,
    Bowman,
    MasterBowman,
    Blowgunman,
    MasterBlowgunman,
    BeastHunter,
    Scout,
    Ranger,
    Hunter,
    Sage,
    Scholar,
    Philosopher,
    Mathematician,
    Historian,
    Astronomer,
    Naturalist,
    Chemist,
    Geographer,
    Scribe,
    Bookbinder,
    Performer,
    Poet,
    Bard,
    Dancer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]

pub enum ColorModification {
    #[default]
    AsIs,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]

pub enum PlantGraphicTemplate {
    StandardLeaves,
    StandardFruit1,
    StandardFruit2,
    StandardFruit3,
    StandardFruit4,
    StandardFlowers1,
    StandardFlowers2,
    StandardFlowers3,
    StandardFlowers4,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]

pub enum GrowthToken {
    Fruit,
    Growth1,
    Growth2,
    Growth3,
    Growth4,
    #[default]
    AsIs,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, specta::Type)]
#[serde(rename_all = "camelCase")]

pub enum GraphicType {
    Creature,
    CreatureCaste,
    StatueCreature,
    StatueCreatureCaste,
    StatuesSurfaceGiant,
    Tile,
    Empty,
    Plant,
    #[default]
    Unknown,
    Template,

    SoilBackground,
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    CustomEdging,
    CustomRamp,
    CustomEdgeW,
    CustomEdgeE,
    CustomEdgeN,
    CustomEdgeS,
    CustomEdgeNW,
    CustomEdgeNE,
    CustomEdgeSW,
    CustomEdgeSE,

    // Interface
    CustomWorkshop,
    ListIcon,

    // The other values
    AddTool,
    Ammo,
    AmmoStraightDefault,
    AmmoStraightWood,
    AmmoDiagonalDefault,
    AmmoDiagonalWood,
    Armor,
    Food,
    Gloves,
    Helm,
    Pants,
    RoughGem,
    ShapeLargeGem,
    ShapeSmallGem,
    Shield,
    ShieldWooden,
    Shoes,
    ShoesMetal,
    SiegeAmmo,
    SiegeAmmoStraightDefault,
    SiegeAmmoStraightWood,
    SiegeAmmoDiagonalDefault,
    SiegeAmmoDiagonalWood,
    Tool,
    ToolWood,
    ToolStone,
    ToolMetal,
    ToolHiveBuilding,
    ToolGlass,
    ToolShape,
    ToolGlassVariant,
    ToolMetalVariant,
    ToolStoneVariant,
    ToolWoodVariant,
    ToolMud,
    ToolWater,
    ToolVomit,
    ToolBlood,
    ToolDamage,
    ToolBands,
    ToolEngraving,
    ToolStuds,
    ToolRings,
    ToolSpikes,
    Toy,
    TrapComponent,
    TrapComponentWeaponTrap,
    TrapComponentUpright1T,
    TrapComponentUpright2T,
    TrapComponentUpright3T,
    TrapComponentUpright4T,
    TrapComponentUpright5T,
    TrapComponentUpright6T,
    TrapComponentUpright7T,
    TrapComponentUpright8T,
    TrapComponentUpright9T,
    TrapComponentUpright10T,
    TrapComponentUpright1B,
    TrapComponentUpright2B,
    TrapComponentUpright3B,
    TrapComponentUpright4B,
    TrapComponentUpright5B,
    TrapComponentUpright6B,
    TrapComponentUpright7B,
    TrapComponentUpright8B,
    TrapComponentUpright9B,
    TrapComponentUpright10B,
    Weapon,
    WeaponDefault,
    WeaponWood,
    WeaponWoodGrown,
    WeaponMaterial,
    WeaponTrap,
    WeaponUpright1T,
    WeaponUpright2T,
    WeaponUpright3T,
    WeaponUpright4T,
    WeaponUpright5T,
    WeaponUpright6T,
    WeaponUpright7T,
    WeaponUpright8T,
    WeaponUpright9T,
    WeaponUpright10T,
    WeaponUpright1B,
    WeaponUpright2B,
    WeaponUpright3B,
    WeaponUpright4B,
    WeaponUpright5B,
    WeaponUpright6B,
    WeaponUpright7B,
    WeaponUpright8B,
    WeaponUpright9B,
    WeaponUpright10B,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]

pub enum TilePageTag {
    TileDim,
    PageDim,
    File,
    #[default]
    Unknown,
}

impl ColorModification {
    pub fn from_token(token: &str) -> Self {
        if let "AS_IS" = token {
            Self::AsIs
        } else {
            warn!("Failed to parse {} as ColorModification", token);
            Self::default()
        }
    }
    pub fn is_default(&self) -> bool {
        matches!(self, Self::AsIs)
    }
}

impl Condition {
    pub fn from_token(token: &str) -> Option<Self> {
        CONDITION_TAGS.get(token).copied()
    }
    pub fn is_default(self) -> bool {
        matches!(self, Self::None)
    }
    /// Used in serialization
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_none(&self) -> bool {
        self.is_default()
    }
}
