use serde::{Deserialize, Serialize};
use tracing::warn;

use super::phf_table::CONDITION_TAGS;

/// A condition that can be applied to a tile/entity
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Condition {
    /// No condition
    None,
    /// The start of a condition
    Condition,
    /// Default condition
    #[default]
    Default,
    /// A condition of "being animated"
    Animated,
    /// Condition of being a corpse
    Corpse,
    /// Condition of being a child
    Child,
    /// Condition of being a baby
    Baby,
    /// Condition of being trained for hunting
    TrainedHunter,
    /// Condition of being trained for war
    TrainedWar,
    /// Condition of being a list icont
    ListIcon,
    /// Condition of being a skeleton
    Skeleton,
    /// Condition of being a skeleton with a skull
    SkeletonWithSkull,
    /// Condition of being a zombie
    Zombie,
    /// Condition of being a necromancer
    Necromancer,
    /// Condition of being male
    Male,
    /// Condition of being female
    Female,
    /// Condition of being a vampire
    VampireCursed,
    /// Condition of being a ghoul
    Ghoul,
    /// Condition of being a disturbed dead
    DisturbedDead,
    /// Condition of being remains
    Remains,
    /// Condition of being a vermin
    Vermin,
    /// Condition of being a light vermin
    LightVermin,
    /// Condition of being a hive
    Hive,
    /// Condition of being a small swarm
    SwarmSmall,
    /// Condition of being a medium swarm
    SwarmMedium,
    /// Condition of being a large swarm
    SwarmLarge,
    /// Condition of being not an artifact
    NotArtifact,
    /// Condition of being a crafted artifact
    CraftedArtifact,
    /// Condition of being dyed
    Dye,
    /// Condition of not being dyed
    NotDyed,
    /// Condition of being a crop
    Crop,
    /// Condition of being a seed
    Seed,
    /// Condition of being a plant (picked)
    Picked,
    /// Condition of being a shrub
    Shrub,
    /// Condition of being a sapling
    Sapling,
    /// Condition of being a crop sprout
    CropSprout,
    /// Condition of being a large crop
    CropL,
    /// Condition of being a medium crop
    CropM,
    /// Condition of being a small crop
    CropR,
    /// Condition of being a dead shrub
    ShrubDead,
    /// Condition of not being a child
    NotChild,
    /// Condition of being at least so many hauled
    HaulCountMin,
    /// Condition of being at most so many hauled
    HaulCountMax,
    /// Condition of being a worn item
    ItemWorn,
    /// Condition of having a profession
    ProfessionCategory,
    /// Condition of being a class
    Class,
    /// Condition of being a syndrome class
    SyndromeClass,
    /// Condition of being a caste
    Caste,
    /// Condition of being a tissue layer
    TissueLayer,
    /// Condition of being a material flag
    MaterialFlag,
    /// Condition of being a material type
    MaterialType,
    /// Condition of being off if an item is present
    ShutOffIfItemPresent,
    /// Condition of being a random part index
    RandomPartIndex,
    /// Condition of being a ghost
    Ghost,
    /// Condition of being a tissue that may have color
    TissueMayHaveColor,
    /// Condition of being a tissue that is at least so long
    TissueMinLength,
    /// Condition of being a tissue that is at most so long
    TissueMaxLength,
    /// Condition of being a tissue at least so curly
    TissueMinCurly,
    /// Condition of being a tissue at most so curly
    TissueMaxCurly,
    /// Condition of being a tissue that may have a shape
    TissueMayHaveShaping,
    /// Condition of being a tissue that is not shaped
    TissueNotShaped,
    /// Condition of being a swapped tissue
    TissueSwap,
    /// Condition of being a specific layer (start layer definition)
    Layer,
    /// Condition of being a specific layer set of layers
    LayerSet,
    /// Condition of being a specific layer group
    LayerGroup,
    /// Condition of being a specific layer group set of layers
    EndLayerGroup,
    /// Condition of being the upper body
    BodyUpper,
    /// Condition of being a copy of a template
    CopyOfTemplate,

    // Professions (somewhat of a hack.. but some mods don't use profession category and instead call direct)
    /// Hammerman profession
    Hammerman,
    /// Master Hammerman profession
    MasterHammerman,
    /// Spearman profession
    Spearman,
    /// Master Spearman profession
    MasterSpearman,
    /// Wrestler profession
    Wrestler,
    /// Master Wrestler profession
    MasterWrestler,
    /// Axeman profession
    Axeman,
    /// Master Axeman profession
    MasterAxeman,
    /// Swordsman profession
    Swordsman,
    /// Master Swordsman profession
    MasterSwordsman,
    /// Maceman profession
    Maceman,
    /// Master Maceman profession
    MasterMaceman,
    /// Pikeman profession
    Pikeman,
    /// Master Pikeman profession
    MasterPikeman,
    /// Recruit profession
    Recruit,
    /// Thief profession
    Thief,
    /// Master Thief profession
    MasterThief,
    /// Lasher profession
    Lasher,
    /// Master Lasher profession
    MasterLasher,
    /// Monster slayer profession
    MonsterSlayer,
    /// Crossbowman profession
    Crossbowman,
    /// Master Crossbowman profession
    MasterCrossbowman,
    /// Bowman profession
    Bowman,
    /// Master Bowman profession
    MasterBowman,
    /// Blowgunman profession
    Blowgunman,
    /// Master Blowgunman profession
    MasterBlowgunman,
    /// Beat hunter profession
    BeastHunter,
    /// Scout profession
    Scout,
    /// Ranger profession
    Ranger,
    /// Hunter profession
    Hunter,
    /// Sage profession
    Sage,
    /// Scholar profession
    Scholar,
    /// Philosopher profession
    Philosopher,
    /// Mathematician profession
    Mathematician,
    /// Historian profession
    Historian,
    /// Astronomer profession
    Astronomer,
    /// Naturalist profession
    Naturalist,
    /// Chemist profession
    Chemist,
    /// Geographer profession
    Geographer,
    /// Scribe profession
    Scribe,
    /// Bookbinder profession
    Bookbinder,
    /// Performer profession
    Performer,
    /// Poet profession
    Poet,
    /// Bard profession
    Bard,
    /// Dancer profession
    Dancer,
}

/// The color modification of the tile
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ColorModification {
    /// The color is as is
    #[default]
    AsIs,
}

/// The graphic of the tile
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlantGraphicTemplate {
    /// The standard leaves
    StandardLeaves,
    /// The standard fruit 1
    StandardFruit1,
    /// The standard fruit 2
    StandardFruit2,
    /// The standard fruit 3
    StandardFruit3,
    /// The standard fruit 4
    StandardFruit4,
    /// The standard flowers 1
    StandardFlowers1,
    /// The standard flowers 2
    StandardFlowers2,
    /// The standard flowers 3
    StandardFlowers3,
    /// The standard flowers 4
    StandardFlowers4,
}

/// The growth token of the tile
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum GrowthToken {
    /// The tile is a fruit
    Fruit,
    /// The tile is growth-1
    Growth1,
    /// The tile is growth-2
    Growth2,
    /// The tile is growth-3
    Growth3,
    /// The tile is growth-4
    Growth4,
    /// The tile is "as-is"
    #[default]
    AsIs,
}

/// The graphic type of the tile
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum GraphicType {
    /// The tile is a creature
    Creature,
    /// The tile is a creature caste
    CreatureCaste,
    /// The tile is a statue of a creature
    StatueCreature,
    /// The tile is a statue of a creature caste
    StatueCreatureCaste,
    /// The tile is a statue of a creature caste with a giant size
    StatuesSurfaceGiant,
    /// A tile
    Tile,
    /// An empty tile
    Empty,
    /// A plant
    Plant,
    /// An unknown type
    #[default]
    Unknown,
    /// A template
    Template,
    /// The tile is soil background
    SoilBackground,
    /// The tile is grass-1
    Grass1,
    /// The tile is grass-2
    Grass2,
    /// The tile is grass-3
    Grass3,
    /// The tile is grass-4
    Grass4,
    /// The tile is custom edging
    CustomEdging,
    /// The tile is custom ramp
    CustomRamp,
    /// The tile is custom corner (W)
    CustomEdgeW,
    /// The tile is custom corner (E)
    CustomEdgeE,
    /// The tile is custom corner (N)
    CustomEdgeN,
    /// The tile is custom corner (S)
    CustomEdgeS,
    /// The tile is custom corner (NW)
    CustomEdgeNW,
    /// The tile is custom corner (NE)
    CustomEdgeNE,
    /// The tile is custom corner (SW)
    CustomEdgeSW,
    /// The tile is custom corner (SE)
    CustomEdgeSE,
    /// The tile is a custom workshop
    CustomWorkshop,
    /// The tile is a list icon
    ListIcon,
    /// The tile is an add tool
    AddTool,
    /// The tile is ammo
    Ammo,
    /// The tile is ammo straight default
    AmmoStraightDefault,
    /// The tile is ammo straight wood
    AmmoStraightWood,
    /// The tile is ammo diagonal default
    AmmoDiagonalDefault,
    /// The tile is ammo diagonal wood
    AmmoDiagonalWood,
    /// The tile is an armor
    Armor,
    /// The tile is food
    Food,
    /// The graphic is of gloves
    Gloves,
    /// The graphic is of a helm
    Helm,
    /// The graphic is of pants
    Pants,
    /// The graphic is of a rough gem
    RoughGem,
    /// The graphic is of a large gem
    ShapeLargeGem,
    /// The graphic is of a small gem
    ShapeSmallGem,
    /// The graphic is of a shield
    Shield,
    /// The graphic is of a wooden shield
    ShieldWooden,
    /// The graphic is of shoes
    Shoes,
    /// The graphic is of metal shoes
    ShoesMetal,
    /// The graphic is of siege ammo
    SiegeAmmo,
    /// The graphic is of siege ammo straight default
    SiegeAmmoStraightDefault,
    /// The graphic is of siege ammo straight wood
    SiegeAmmoStraightWood,
    /// The graphic is of siege ammo diagonal default
    SiegeAmmoDiagonalDefault,
    /// The graphic is of siege ammo diagonal wood
    SiegeAmmoDiagonalWood,
    /// The graphic is of a tool
    Tool,
    /// The graphic is of a tool wood
    ToolWood,
    /// The graphic is of a tool stone
    ToolStone,
    /// The graphic is of a tool metal
    ToolMetal,
    /// The graphic is of a beehive
    ToolHiveBuilding,
    /// The graphic is of a glass tool
    ToolGlass,
    /// The graphic is of a tool shape
    ToolShape,
    /// The graphic is a glass tool variant
    ToolGlassVariant,
    /// The graphic is a metal tool variant
    ToolMetalVariant,
    /// The graphic is a stone tool variant
    ToolStoneVariant,
    /// The graphic is a wood tool variant
    ToolWoodVariant,
    /// The graphic is a mud tool
    ToolMud,
    /// The graphic is a water tool
    ToolWater,
    /// The graphic is a vomit tool
    ToolVomit,
    /// The graphic is a blood tool
    ToolBlood,
    /// The graphic is a plant tool
    ToolDamage,
    /// The graphic is a tool with binds on it
    ToolBands,
    /// The graphic is a tool with engravings
    ToolEngraving,
    /// The graphic is a tool with studs
    ToolStuds,
    /// The graphic is a tool with rings
    ToolRings,
    /// The graphic is a tool with spikes
    ToolSpikes,
    /// The graphic is a toy
    Toy,
    /// The graphic is a trap component
    TrapComponent,
    /// The graphic is a weapon trap
    TrapComponentWeaponTrap,
    /// The graphic is a weapon trap upright 1-T
    TrapComponentUpright1T,
    /// The graphic is a weapon trap upright 2-T
    TrapComponentUpright2T,
    /// The graphic is a weapon trap upright 3-T
    TrapComponentUpright3T,
    /// The graphic is a weapon trap upright 4-T
    TrapComponentUpright4T,
    /// The graphic is a weapon trap upright 5-T
    TrapComponentUpright5T,
    /// The graphic is a weapon trap upright 6-T
    TrapComponentUpright6T,
    /// The graphic is a weapon trap upright 7-T
    TrapComponentUpright7T,
    /// The graphic is a weapon trap upright 8-T
    TrapComponentUpright8T,
    /// The graphic is a weapon trap upright 9-T
    TrapComponentUpright9T,
    /// The graphic is a weapon trap upright 10-T
    TrapComponentUpright10T,
    /// The graphic is a weapon trap upright 1-B
    TrapComponentUpright1B,
    /// The graphic is a weapon trap upright 2-B
    TrapComponentUpright2B,
    /// The graphic is a weapon trap upright 3-B
    TrapComponentUpright3B,
    /// The graphic is a weapon trap upright 4-B
    TrapComponentUpright4B,
    /// The graphic is a weapon trap upright 5-B
    TrapComponentUpright5B,
    /// The graphic is a weapon trap upright 6-B
    TrapComponentUpright6B,
    /// The graphic is a weapon trap upright 7-B
    TrapComponentUpright7B,
    /// The graphic is a weapon trap upright 8-B
    TrapComponentUpright8B,
    /// The graphic is a weapon trap upright 9-B
    TrapComponentUpright9B,
    /// The graphic is a weapon trap upright 10-B
    TrapComponentUpright10B,
    /// The graphic is a weapon
    Weapon,
    /// The graphic is a weapon default
    WeaponDefault,
    /// The graphic is a weapon made of wood
    WeaponWood,
    /// The graphic is a weapon made of grown wood
    WeaponWoodGrown,
    /// The graphic is a weapon made of material
    WeaponMaterial,
    /// The graphic is of a weapon used in traps
    WeaponTrap,
    /// The graphic is of a weapon upright 1-T
    WeaponUpright1T,
    /// The graphic is of a weapon upright 2-T
    WeaponUpright2T,
    /// The graphic is of a weapon upright 3-T
    WeaponUpright3T,
    /// The graphic is of a weapon upright 4-T
    WeaponUpright4T,
    /// The graphic is of a weapon upright 5-T
    WeaponUpright5T,
    /// The graphic is of a weapon upright 6-T
    WeaponUpright6T,
    /// The graphic is of a weapon upright 7-T
    WeaponUpright7T,
    /// The graphic is of a weapon upright 8-T
    WeaponUpright8T,
    /// The graphic is of a weapon upright 9-T
    WeaponUpright9T,
    /// The graphic is of a weapon upright 10-T
    WeaponUpright10T,
    /// The graphic is of a weapon upright 1-B
    WeaponUpright1B,
    /// The graphic is of a weapon upright 2-B
    WeaponUpright2B,
    /// The graphic is of a weapon upright 3-B
    WeaponUpright3B,
    /// The graphic is of a weapon upright 4-B
    WeaponUpright4B,
    /// The graphic is of a weapon upright 5-B
    WeaponUpright5B,
    /// The graphic is of a weapon upright 6-B
    WeaponUpright6B,
    /// The graphic is of a weapon upright 7-B
    WeaponUpright7B,
    /// The graphic is of a weapon upright 8-B
    WeaponUpright8B,
    /// The graphic is of a weapon upright 9-B
    WeaponUpright9B,
    /// The graphic is of a weapon upright 10-B
    WeaponUpright10B,
}

/// The tokens used to define the tile page
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TilePageTag {
    /// The dimensions of the tile
    TileDim,
    /// The dimensions of the page
    PageDim,
    /// The file path
    File,
    /// An unknown token
    #[default]
    Unknown,
}

impl ColorModification {
    /// Parse a token into a `ColorModification`
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    ///
    /// # Returns
    ///
    /// The parsed `ColorModification`
    #[must_use]
    pub fn from_token(token: &str) -> Self {
        if token == "AS_IS" {
            Self::AsIs
        } else {
            warn!("Failed to parse {} as ColorModification", token);
            Self::default()
        }
    }
    /// Whether the `ColorModification` is the default value.
    ///
    /// # Returns
    ///
    /// True if the `ColorModification` is the default value, false otherwise.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::AsIs)
    }
}

impl Condition {
    /// Parse a token into a Condition
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    ///
    /// # Returns
    ///
    /// The parsed Condition
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        CONDITION_TAGS.get(token).copied()
    }
    /// Whether the Condition is the default value.
    ///
    /// # Returns
    ///
    /// True if the Condition is the default value, false otherwise.
    #[must_use]
    pub const fn is_default(self) -> bool {
        matches!(self, Self::None)
    }
    /// Whether the Condition is the default value.
    ///
    /// # Returns
    ///
    /// True if the Condition is the default value, false otherwise.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.is_default()
    }
}
