use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum MaterialType {
    Inorganic,
    Stone,
    Metal,
    Coal,
    CreatureMaterial,
    LocalCreatureMaterial,
    PlantMaterial,
    LocalPlantMaterial,
    GetMaterialFromReagent,
    // Special "Hardcoded" Materials
    // Inorganic -> Magma
    Amber,
    Coral,
    GlassGreen,
    GlassClear,
    GlassCrystal,
    Water,
    // Coal -> Coal
    Potash,
    Ash,
    PearlAsh,
    Lye,
    Mud,
    Vomit,
    Salt,
    /// Brown Filth
    FilthB,
    /// Yellow Filth
    FilthY,
    UnknownSubstance,
    Grime,
    #[default]
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum FuelType {
    Charcoal,
    Coke,
    NoMaterialGloss,
    /// None is an invalid option, so its a hint that this is not set.
    #[default]
    None,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum MaterialState {
    Solid,
    Liquid,
    Gas,
    /// POWDER or SOLID_POWDER
    Powder,
    /// PASTE or SOLID_PASTE
    Paste,
    /// PRESSED or SOLID_PRESSED
    Pressed,
    /// Default value is invalid, so its a hint that this is not set.
    #[default]
    Unknown,
    /// Denotes all possible material states
    All,
    /// Denotes 'Solid', 'Powder', 'Paste', and 'Pressed'
    AllSolid,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum MaterialUsage {
    /// Lets the game know that an animal was likely killed in the production of this item.
    /// Entities opposed to killing animals (Elves in vanilla) will refuse to accept these items in trade.
    ImpliesAnimalKill,
    /// Classifies the material as plant-based alcohol, allowing its storage in food stockpiles under "Drink (Plant)".
    AlcoholPlant,
    /// Classifies the material as animal-based alcohol, allowing its storage in food stockpiles under "Drink (Animal)".
    AlcoholCreature,
    /// Classifies the material as generic alcohol. Implied by both ALCOHOL_PLANT and ALCOHOL_CREATURE. Exact behavior unknown, possibly vestigial.
    Alcohol,
    /// Classifies the material as plant-based cheese, allowing its storage in food stockpiles under "Cheese (Plant)".
    CheesePlant,
    /// Classifies the material as animal-based cheese, allowing its storage in food stockpiles under "Cheese (Animal)".
    CheeseCreature,
    /// Classifies the material as generic cheese. Implied by both CHEESE_PLANT and CHEESE_CREATURE. Exact behavior unknown, possibly vestigial.
    Cheese,
    /// Classifies the material as plant powder, allowing its storage in food stockpiles under "Milled Plant".
    PowderMiscPlant,
    /// Classifies the material as creature powder, allowing its storage in food stockpiles under "Bone Meal".
    /// Unlike milled plants, such as sugar and flour, "Bone Meal" barrels or pots may not contain bags.
    /// Custom reactions using this product better use buckets or jugs instead.
    PowderMiscCreature,
    /// Classifies the material as generic powder. Implied by both POWDER_MISC_PLANT and POWDER_MISC_CREATURE.
    /// Exact behavior unknown, possibly vestigial.
    PowderMisc,
    /// Permits globs of the material in solid form to be stored in food stockpiles under "Fat" - without it,
    /// dwarves will come by and "clean" the items, destroying them (unless [DO_NOT_CLEAN_GLOB] is also included).
    StockpileGlobOrStockpileGlobSolid,
    /// Classifies the material as milled paste, allowing its storage in food stockpiles under "Paste".
    StockpileGlobPaste,
    /// Classifies the material as pressed goods, allowing its storage in food stockpiles under "Pressed Material".
    StockpileGlobPressed,
    /// Classifies the material as a plant growth (e.g. fruits, leaves), allowing its storage in food stockpiles under Plant Growth/Fruit.
    StockpilePlantGrowth,
    /// Classifies the material as a plant extract, allowing its storage in food stockpiles under "Extract (Plant)".
    LiquidMiscPlant,
    /// Classifies the material as a creature extract, allowing its storage in food stockpiles under "Extract (Animal)".
    LiquidMiscCreature,
    /// Classifies the material as a miscellaneous liquid, allowing its storage in food stockpiles under "Misc. Liquid" along with lye.
    LiquidMiscOther,
    /// Classifies the material as a generic liquid. Implied by LIQUID_MISC_PLANT, LIQUID_MISC_CREATURE, and LIQUID_MISC_OTHER. Exact behavior unknown, possibly vestigial.
    LiquidMisc,
    /// Classifies the material as a plant, allowing its storage in food stockpiles under "Plants".
    StructuralPlantMat,
    /// Classifies the material as a plant seed, allowing its storage in food stockpiles under "Seeds".
    SeedMat,
    /// Classifies the material as bone, allowing its use for bone carvers and restriction from stockpiles by material.
    Bone,
    /// Classifies the material as wood, allowing its use for carpenters and storage in wood stockpiles. Entities opposed to killing plants (i.e. Elves) will refuse to accept these items in trade.
    Wood,
    /// Classifies the material as plant fiber, allowing its use for clothiers and storage in cloth stockpiles under "Thread (Plant)" and "Cloth (Plant)".
    ThreadPlant,
    /// Classifies the material as tooth, allowing its use for bone carvers and restriction from stockpiles by material.
    Tooth,
    /// Classifies the material as horn, allowing its use for bone carvers and restriction from stockpiles by material.
    Horn,
    /// Classifies the material as hair, allowing for its use for spinners and restriction from refuse stockpiles by material.
    Hair,
    /// Classifies the material as pearl, allowing its use for bone carvers and restriction from stockpiles by material.
    Pearl,
    /// Classifies the material as shell, allowing its use for bone carvers and restriction from stockpiles by material.
    Shell,
    /// Classifies the material as leather, allowing its use for leatherworkers and storage in leather stockpiles.
    Leather,
    /// Classifies the material as silk, allowing its use for clothiers and storage in cloth stockpiles under "Thread (Silk)" and "Cloth (Silk)".
    Silk,
    /// Classifies the material as soap, allowing it to be used as a bath detergent and stored in bar/block stockpiles under "Bars: Other Materials".[Verify]
    Soap,
    /// Material generates miasma when it rots.
    GeneratesMiasma,
    /// Classifies the material as edible meat.[Verify]
    Meat,
    /// Material will rot if not stockpiled appropriately. Currently only affects food and refuse, other items made of this material will not rot.
    Rots,
    /// In most living creatures, it controls many bodily functions and movements by sending signals around the body. See: Nervous tissue
    NervousTissue,
    /// Tells the game to classify contaminants of this material as being "blood" in Adventurer mode tile descriptions ("Here we have a Dwarf in a slurry of blood.").
    BloodMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "ichor".
    IchorMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "goo".
    GooMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "slime".
    SlimeMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "pus".
    PusMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "sweat".
    SweatMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "tears".
    TearsMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "spit".
    SpitMapDescriptor,
    /// Contaminants composed of this material evaporate over time, slowly disappearing from the map. Used internally by water.
    Evaporates,
    /// Used for materials which cause syndromes, causes it to enter the creature's blood instead of simply spattering on the surface.
    EntersBlood,
    /// Can be eaten by vermin.
    EdibleVermin,
    /// Can be eaten raw.
    EdibleRaw,
    /// Can be cooked and then eaten.
    EdibleCooked,
    /// Prevents globs made of this material from being cleaned up and destroyed.
    DoNotCleanGlob,
    /// Prevents the material from showing up in Stone stockpile settings.
    NoStoneStockpile,
    /// The material can be made into minecarts, wheelbarrows, and stepladders at the metalsmith's forge.
    ItemsMetal,
    /// Equivalent to ITEMS_HARD. Given to bone.
    ItemsBarred,
    /// Equivalent to ITEMS_HARD. Given to shell.
    ItemsScaled,
    /// Equivalent to ITEMS_SOFT. Given to leather.
    ItemsLeather,
    /// The material can be made into clothing, amulets, bracelets, earrings, backpacks, and quivers, contingent
    /// on which workshops accept the material. Given to plant fiber, silk and wool.
    ItemsSoft,
    /// The material can be made into furniture, crafts, mechanisms, and blocks, contingent on which workshops accept the material.
    /// Random crafts made from this material include all seven items. Given to stone, wood, bone, shell, chitin, claws, teeth,
    /// horns, hooves and beeswax. Hair, pearls and eggshells also have the tag.
    ItemsHard,
    /// Used to define that the material is a stone. Allows its usage in masonry and stonecrafting and storage in stone stockpiles, among other effects.
    IsStone,
    /// Defines the material is a ceramic.
    IsCeramic,
    /// Used for a stone that cannot be dug into.
    Undiggable,
    /// Causes containers made of this material to be prefixed with "unglazed" if they have not yet been glazed.
    DisplayUnglazed,
    /// Classifies the material as yarn, allowing its use for clothiers and its storage in cloth stockpiles under "Thread (Yarn)" and "Cloth (Yarn)".
    Yarn,
    /// Classifies the material as metal thread, permitting thread and cloth to be stored in cloth stockpiles under "Thread (Metal)" and "Cloth (Metal)".
    StockpileThreadMetal,
    /// Defines the material as being metal, allowing it to be used at forges.
    IsMetal,
    /// Used internally by green glass, clear glass, and crystal glass. Appears to only affect the [GLASS_MATERIAL] reaction token. Does not cause the game
    /// to treat the material like glass, i.e being referred to as "raw" instead of "rough" in its raw form or being displayed in the "glass" trade/embark category.
    IsGlass,
    /// Can be used in the production of crystal glass.
    CrystalGlassable,
    /// Melee weapons can be made out of this material.
    ItemsWeapon,
    /// Ranged weapons can be made out of this material.
    ItemsWeaponRanged,
    /// Anvils can be made out of this material.
    ItemsAnvil,
    /// Ammunition can be made out of this material.
    ItemsAmmo,
    /// Picks can be made out of this material.
    ItemsDigger,
    /// Armor can be made out of this material.
    ItemsArmor,
    /// Used internally by amber and coral. Functionally equivalent to ITEMS_HARD.
    ItemsDelicate,
    /// Siege engine parts can be made out of this material. Does not appear to work.
    ItemsSiegeEngine,
    /// Querns and millstones can be made out of this material. Does not appear to work.
    ItemsQuern,
    #[default]
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum MaterialProperty {
    /// Imports the properties of the specified preexisting material template.
    UseMaterialTemplate,
    /// Applies a prefix to all items made from the material. For PLANT and CREATURE materials, this defaults to the plant/creature name.
    /// Not permitted in material template definitions.
    Prefix,
    /// Overrides the name of BOULDER items (i.e. mined-out stones) made of the material (used for native copper/silver/gold/platinum
    /// to make them be called "nuggets" instead of "boulders").
    StoneName,
    /// Used to indicate that said material is a gemstone - when tiles are mined out, rough gems will be yielded instead of boulders.
    /// Plural can be "STP" to automatically append an "s" to the singular form, and OVERWRITE_SOLID will override the relevant STATE_NAME and STATE_ADJ values.
    IsGem,
    /// Specifies what the material should be treated as when drinking water contaminated by it, for generating unhappy thoughts.
    /// Valid values are BLOOD, SLIME, VOMIT, ICHOR, PUS, GOO, GRIME, and FILTH.
    TempDietInfo,
    /// Allows the material to be used as dye, and defines color of dyed items.
    PowderDye,
    /// Specifies the tile that will be used to represent unmined tiles made of this material. Generally only used with stones. Defaults to 219 ('█').
    Tile,
    /// Specifies the tile that will be used to represent BOULDER items made of this material. Generally only used with stones. Defaults to 7 ('•').
    ItemSymbol,
    /// The on-screen color of the material. Uses a standard 3-digit color token. Equivalent to [TILE_COLOR:a:b:c], [BUILD_COLOR:b:a:X] (X = 1 if 'a' equals 'b', 0 otherwise), and [BASIC_COLOR:a:c]
    DisplayColor,
    /// The color of objects made of this material which use both the foreground and background color: doors, floodgates, hatch covers, bins, barrels, and cages. Defaults to 7:7:1 (white).
    BuildColor,
    /// The color of unmined tiles containing this material (for stone and soil), as well as engravings in this material. Defaults to 7:7:1 (white).
    TileColor,
    /// The color of objects made of this material which use only the foreground color, including workshops, floors and boulders, and smoothed walls. Defaults to 7:1 (white).
    BasicColor,
    /// Determines the color of the material at the specified state. See below for a list of valid material states. Color comes from descriptor_color_standard.txt.
    /// The nearest color value is used to display contaminants and body parts made of this material in ASCII and to color items and constructions made from this material with graphics.
    /// Example:[STATE_COLOR:ALL_SOLID:GRAY]
    StateColor,
    /// Determines the name of the material at the specified state, as displayed in-game.[STATE_NAME:ALL_SOLID:stone]
    StateName,
    /// Like STATE_NAME, but used in different situations. Equipment made from the material uses the state adjective and not the state name.
    StateAdjective,
    /// Sets both STATE_NAME and STATE_ADJ at the same time.
    StateNameAdjective,
    /// The material's tendency to absorb liquids. Containers made of materials with nonzero absorption cannot hold liquids unless they have been glazed.
    /// Defaults to 0.
    Absorption,
    /// Specifies how hard of an impact (in kilopascals) the material can withstand before it will start deforming permanently.
    /// Used for blunt-force combat. Defaults to 10000.
    ImpactYield,
    /// Specifies how hard of an impact the material can withstand before it will fail entirely. Used for blunt-force combat. Defaults to 10000.
    ImpactFracture,
    /// Specifies how much the material will have given (in parts-per-100000) when the yield point is reached. Used for blunt-force combat. Defaults to 0.
    /// Apparently affects in combat whether the corresponding tissue is bruised (value >= 50000), torn (value between 25000 and 49999), or fractured (value <= 24999)
    ImpactElasticity,
    /// Specifies how hard the material can be compressed before it will start deforming permanently. Determines a tissue's resistance to pinching and response to strangulation. Defaults to 10000.
    CompressiveYield,
    /// Specifies how hard the material can be compressed before it will fail entirely. Determines a tissue's resistance to pinching and response to strangulation. Defaults to 10000.
    CompressiveFracture,
    /// Specifies how much the material will have given when it has been compressed to its yield point. Determines a tissue's resistance to pinching and response to strangulation. Defaults to 0.
    CompressiveElasticity,
    /// Specifies how hard the material can be stretched before it will start deforming permanently. Determines a tissue's resistance to a latching and tearing bite. Defaults to 10000.
    TensileYield,
    /// Specifies how hard the material can be stretched before it will fail entirely. Determines a tissue's resistance to a latching and tearing bite. Defaults to 10000.
    TensileFracture,
    /// Specifies how much the material will have given when it is stretched to its yield point. Determines a tissue's resistance to a latching and tearing bite. Defaults to 0.
    TensileElasticity,
    /// Specifies how hard the material can be twisted before it will start deforming permanently. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to 10000.
    TorsionYield,
    /// Specifies how hard the material can be twisted before it will fail entirely. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to 10000.
    TorsionFracture,
    /// Specifies how much the material will have given when it is twisted to its yield point. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to 0.
    TorsionElasticity,
    /// Specifies how hard the material can be sheared before it will start deforming permanently. Used for cutting calculations. Defaults to 10000.
    ShearYield,
    /// Specifies how hard the material can be sheared before it will fail entirely. Used for cutting calculations. Defaults to 10000.
    ShearFracture,
    /// Specifies how much the material will have given when sheared to its yield point. Used for cutting calculations. Defaults to 0.
    ShearElasticity,
    /// Specifies how hard the material can be bent before it will start deforming permanently. Determines a tissue's resistance to being mangled with a joint lock. Defaults to 10000.
    BendingYield,
    /// Specifies how hard the material can be bent before it will fail entirely. Determines a tissue's resistance to being mangled with a joint lock. Defaults to 10000.
    BendingFracture,
    /// Specifies how much the material will have given when bent to its yield point. Determines a tissue's resistance to being mangled with a joint lock. Defaults to 0.
    BendingElasticity,
    /// How sharp the material is. Used in cutting calculations. Applying a value of at least 10000 to a stone will allow weapons to be made from that stone. Defaults to 10000.
    MaxEdge,
    /// Value modifier for the material. Defaults to 1. This number can be made negative by placing a "-" in front, resulting in things that you are paid to buy and must pay to sell.
    MaterialValue,
    /// Multiplies the value of the material. Not permitted in material template definitions.
    MultiplyValue,
    /// Rate at which the material heats up or cools down (in joules/kilogram-kelvin). If set to NONE, the temperature will be fixed at its initial value.
    /// Defaults to NONE.
    SpecificHeat,
    /// Temperature above which the material takes damage from heat. Defaults to NONE.
    /// If the material has an ignite point but no heatdam point, it will burn for a very long time (9 months and 16.8 days).
    HeatDamagePoint,
    /// Temperature below which the material takes damage from cold. Defaults to NONE.
    ColdDamagePoint,
    /// Temperature at which the material will catch fire. Defaults to NONE.
    IgnitionPoint,
    /// Temperature at which the material melts. Defaults to NONE.
    MeltingPoint,
    /// Temperature at which the material boils. Defaults to NONE.
    BoilingPoint,
    /// Items composed of this material will initially have this temperature.
    /// Used in conjunction with [SPEC_HEAT:NONE] to make material's temperature fixed at the specified value.
    /// Defaults to NONE.
    MaterialFixedTemperature,
    /// Changes a material's HEATDAM_POINT, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetHeatDamagePoint,
    /// Changes a material's COLDDAM_POINT, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetColdDamagePoint,
    /// Changes a material's IGNITE_POINT, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetIgnitePoint,
    /// Changes a material's MELTING_POINT, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetMeltingPoint,
    /// Changes a material's BOILING_POINT, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetBoilingPoint,
    /// Changes a material's MAT_FIXED_TEMP, but only if it was not set to NONE. Not permitted in material template definitions.
    IfExistsSetMatFixedTemp,
    /// Specifies the density (in kilograms per cubic meter) of the material when in solid form. Also affects combat calculations;
    /// affects blunt-force damage and ability of weak-in-impact-yield blunt attacks to pierce armor. Defaults to NONE.
    SolidDensity,
    /// Specifies the density of the material when in liquid form. Defaults to NONE. Also affects combat calculations;
    /// affects blunt force damage like SOLID_DENSITY, but only for attacks made by liquids (e.g. forgotten beasts made of water).
    LiquidDensity,
    /// Specifies (in kg/mol) the molar mass of the material in gaseous form. Also affects combat calculations like the densities,
    /// but only for attacks made by gases (e.g. forgotten beasts made of steam).
    MolarMass,
    /// Specifies the type of container used to store the material. Used in conjunction with the [EXTRACT_BARREL], [EXTRACT_VIAL], or [EXTRACT_STILL_VIAL] plant tokens.
    /// Defaults to BARREL.
    ExtractStorage,
    /// Specifies the item type used for butchering results made of this material. Stock raws use GLOB:NONE for fat and MEAT:NONE for other meat materials.
    ButcherSpecial,
    /// When a creature is butchered, meat yielded from organs made from this material will be named via this token.
    MeatName,
    /// Specifies the name of blocks made from this material.
    BlockName,
    /// The material forms "wafers" instead of "bars".
    Wafers,
    /// Used with reaction raws to associate a reagent material with a product material. The first argument is used by HAS_MATERIAL_REACTION_PRODUCT and GET_MATERIAL_FROM_REAGENT in reaction raws.
    /// The remainder is a material reference, generally LOCAL_CREATURE_MAT:SUBTYPE or LOCAL_PLANT_MAT:SUBTYPE or INORGANIC:STONETYPE.[MATERIAL_REACTION_PRODUCT:TAN_MAT:LOCAL_CREATURE_MAT:LEATHER]
    MaterialReactionProduct,
    /// Used with reaction raws to associate a reagent material with a complete item. The first argument is used by HAS_ITEM_REACTION_PRODUCT and GET_ITEM_DATA_FROM_REAGENT in reaction raws.
    /// The rest refers to the type of item, then its material.[ITEM_REACTION_PRODUCT:BAG_ITEM:PLANT_GROWTH:LEAVES:LOCAL_PLANT_MAT:LEAF]
    ItemReactionProduct,
    /// "Used to classify all items made of the material, so that reactions can use them as generic reagents.In default raws, the following are used:
    /// `FAT`, `TALLOW`, `SOAP`, `PARCHMENT`, `PAPER_PLANT`, `PAPER_SLURRY`, `MILK`, `CHEESE`, `WAX`.
    /// `CAN_GLAZE` - items made from this material can be glazed.
    /// `FLUX` - can be used as flux in pig iron and steel making.
    /// `GYPSUM` - can be processed into gypsum plaster.
    /// `CALCIUM_CARBONATE` - can be used in production of quicklime."
    ReactionClass,
    /// "Makes BOULDER acceptable as a reagent in reactions that require ""METAL_ORE:MATERIAL_NAME"", as well as smelting directly into metal bars.
    /// Places the material under ""Metal Ores"" in Stone stockpiles." The specified value determines the probability for this product (see Tetrahedrite or Galena for details).
    MetalOre,
    /// Makes BOULDER items made of the material acceptable for strand extraction into threads; see also STOCKPILE_THREAD_METAL.
    /// Value presumably determines the probability of this product extracted.[Verify]
    ThreadMetal,
    /// Allows the material to be used to make casts.
    HardensWithWater,
    /// Determines effectiveness of soap - if the amount of grime on a body part is more than 3-SOAP_LEVEL, it sets it to 3-SOAP_LEVEL; as such setting it above 3 is bad. Soap has [SOAP_LEVEL:2]. Defaults to 0.
    SoapLevel,
    /// Begins defining a syndrome applied by the material. Multiple syndromes can be specified. See Syndrome token.
    Syndrome,
    /// This is since .50 in the raws of several antler-wielding animals. It is used to show an antler as bodypart.
    Antler,
    /// For default value, use unknown.
    #[default]
    Unknown,
}

impl FuelType {
    /// Used just to help serialization
    pub fn is_default(&self) -> bool {
        matches!(self, FuelType::None)
    }
}
impl MaterialType {
    /// Used just to help serialization
    pub fn is_default(&self) -> bool {
        matches!(self, MaterialType::Unknown)
    }
}
