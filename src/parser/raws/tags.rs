use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CreatureTag {
    ArtificialHiveable,
    DoesNotExist,
    Evil,
    Fanciful,
    Good,
    Savage,
    Generated,
    Ubiquitous,
    VerminSoil,
    VerminSoilColony,
    VerminRotter,
    VerminGrounder,
    VerminFish,
    VerminEater,
    LargeRoaming,
    LocalPopsControllable,
    LocalPopsProduceHeroes,
    LooseClusters,
    Mundane,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InorganicTag {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PlantTag {
    Dry,
    Evil,
    Good,
    Savage,
    Wet,
    Spring,
    Summer,
    Autumn,
    Winter,
    Brewable,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ContaminateTag {
    Blood,
    Slime,
    Vomit,
    Ichor,
    Pus,
    Goo,
    Grime,
    Filth,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MaterialStateTag {
    Solid,
    Liquid,
    Gas,
    Powder,
    Paste,
    Pressed,
    All,
    AllSolid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MaterialTag {
    Alcohol,
    AlcoholCreature,
    AlcoholPlant,
    Bone,
    Cheese,
    CheeseCreature,
    CheesePlant,
    CrystalGlassable,
    DisplayUnglazed,
    DoNotCleanGlob,
    EdibleCooked,
    EdibleRaw,
    EdibleVermin,
    EntersBlood,
    Evaporates,
    Feather,
    GeneratesMiasma,
    Horn,
    Hoof,
    ImpliesAnimalKill,
    IsCeramic,
    IsGlass,
    IsMetal,
    IsStone,
    ItemsAmmo,
    ItemsAnvil,
    ItemsArmor,
    ItemsBarred,
    ItemsDelicate,
    ItemsDigger,
    ItemsHard,
    ItemsLeather,
    ItemsMetal,
    ItemsQuern,
    ItemsScaled,
    ItemsSiegeEngine,
    ItemsSoft,
    ItemsWeapon,
    ItemsWeaponRanged,
    Leather,
    LiquidMisc,
    LiquidMiscCreature,
    LiquidMiscOther,
    LiquidMiscPlant,
    MapDescriptorBlood,
    MapDescriptorGoo,
    MapDescriptorIchor,
    MapDescriptorPus,
    MapDescriptorSlime,
    MapDescriptorSpit,
    MapDescriptorSweat,
    MapDescriptorTears,
    Meat,
    NoStoneStockpile,
    Pearl,
    PowderMisc,
    PowderMiscCreature,
    PowderMiscPlant,
    Rots,
    SeedMaterial,
    Shell,
    Silk,
    Soap,
    StockpileGlob,
    StockpileGlobPaste,
    StockpileGlobPressed,
    StockpileGlobSolid,
    StockpilePlantGrowth,
    StockpileThreadMaterial,
    StructuralPlantMaterial,
    ThreadPlant,
    Tooth,
    Undiggable,
    Wafers,
    Wood,
    Yarn,
    Sedimentary,
    Aquifer,
    SedimentaryOceanShallow,
    SedimentaryOceanDeep,
    IgneousIntrusive,
    IgneousExtrusive,
    Metamorphic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SyndromeTag {
    Contact,
    Ingested,
    Inhaled,
    Injected,
    NoHospital,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectTag {
    Localized,
    OnlyVascular,
    OnlyMuscular,
    SizeDilutes,
    SizeDelays,
    CanBeHidden,
    Resistible, // Resist​able in raws
    Abrupt,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EffectConditionTag {
    Bruising,
    Blisters,
    Oozing,
    Bleeding,
    Swelling,
    Necrosis,
    Numbness,
    Pain,
    Paralysis,
    ImpairFunction,
    Dizziness,
    Drowsiness,
    Unconsciousness,
    Fever,
    Nausea,
    CoughBlood,
    VomitBlood,

    ReducePain,
    ReduceSwelling,
    ReduceParalysis,
    ReduceDizziness,
    ReduceNausea,
    ReduceFever,
    StopBleeding,
    CloseOpenWounds,
    CureInfection,
    HealTissues,
    HealNerves,
    RegrowParts,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CasteTag {
    LaysEggs,
    AmbushPredator,
    Amphibious,
    AdoptsOwner,
    Benign,
    Aquatic,
    ArenaRestricted,
    AtPeaceWithWildlife,
    BoneCarn,
    CanLearn,
    CanSpeak,
    Carnivore,
    CommonDomestic,
    CookableLive,
    Demon,
    DieWhenVerminBite,
    Equips,
    Extravision,
    FeatureBeast,
    Female,
    FireImmune,
    FireimmuneSuper,
    FishItem,
    Flier,
    Gnawer,
    HasNerves,
    HuntsVermin,
    Immobile,
    ImmobileLand,
    Immolate,
    Intelligent,
    LightGen,
    LockPicker,
    MagmaVision,
    Male,
    Meanderer,
    Megabeast,
    Mischievous,
    Mount,
    MountExotic,
    MultipartFullVision,
    MultipleLitterRare,
    Natural,
    NoConnectionsForMovement,
    NoDizziness,
    NoDrink,
    NoEat,
    NoFevers,
    NoGender,
    NoSleep,
    NoBones,
    NoBreathe,
    NoEmotion,
    NoExert,
    NoFear,
    NoMeat,
    NoNausea,
    NoPain,
    NoSkin,
    NoSkull,
    NoSmellyRot,
    NoStuckIns,
    NoStrun,
    NotButcherable,
    NotLiving,
    NoThought,
    OpposedToLife,
    OutsiderControllable,
    PackAnimal,
    ParalyzeImmune,
    Pet,
    PetExotic,
    Power,
    SemiMegabeast,
    SlowLearner,
    SmallRemains,
    StandardGrazer, //Acts as [GRAZER] but set to 20000*G*(max size)^(-3/4)
    Supernatural,
    SwimsInnate,
    SwimsLearned,
    ThickWeb,
    Titan,
    Trances,
    TrapAvoid,
    UniqueDemon,
    Vegetation,
    VerminHateable,
    VerminMicro,
    VerminNofish,
    VerminNoroam,
    VerminNotrap,
    WagonPuller,
    WebImmune,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFBodySize {
    years: u32,
    days: u32,
    size_cm3: u32,
}
impl DFBodySize {
    pub const fn new(years: u32, days: u32, size_cm3: u32) -> Self {
        Self {
            years,
            days,
            size_cm3,
        }
    }
    pub fn size_cm3(&self) -> u32 {
        self.size_cm3
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFMilkable {
    material: String,
    frequency: u32,
}
impl DFMilkable {
    pub fn new(material: &str, frequency: u32) -> Self {
        Self {
            material: String::from(material),
            frequency,
        }
    }
    // Helper for emptiness
    pub fn is_empty(&self) -> bool {
        self.material.len() == 0 && self.frequency == 0
    }
}

// active time:
//      diurnal & nocturnal & crepuscular & matutinal & vespertine = 31
pub static ACTIVE_DIURNAL: u8 = 1;
pub static ACTIVE_NOCTURNAL: u8 = 2;
pub static ACTIVE_CREPUSCULAR: u8 = 4;
pub static ACTIVE_MATUTINAL: u8 = 8;
pub static ACTIVE_VESPERTINE: u8 = 16;

// curious beast:
//      eater & guzzler & item = 7
pub static CURIOUS_EATER: u8 = 1;
pub static CURIOUS_GUZZLER: u8 = 2;
pub static CURIOUS_ITEM: u8 = 4;

// "no" season (creature does not appear):
//      NO_SPRING & NO_SUMMER & NO_AUTUMN & NO_WINTER = 15
pub static NO_SPRING: u8 = 1;
pub static NO_SUMMER: u8 = 2;
pub static NO_FALL: u8 = 4;
pub static NO_WINTER: u8 = 8;

// trainable:
//      war & hunting = 3
pub static TRAINABLE_HUNTING: u8 = 1;
pub static TRAINABLE_WAR: u8 = 2;
