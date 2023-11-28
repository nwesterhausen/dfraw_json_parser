use super::tokens::{FuelType, MaterialProperty, MaterialType, MaterialUsage};
pub static MATERIAL_TYPE_TOKENS: phf::Map<&'static str, MaterialType> = phf::phf_map! {
    "INORGANIC" => MaterialType::Inorganic,
    "STONE" => MaterialType::Stone,
    "METAL" => MaterialType::Metal,
    "COAL" => MaterialType::Coal,
    "CREATURE_MAT" => MaterialType::CreatureMaterial,
    "LOCAL_CREATURE_MAT" => MaterialType::LocalCreatureMaterial,
    "PLANT_MAT" => MaterialType::PlantMaterial,
    "LOCAL_PLANT_MAT" => MaterialType::LocalPlantMaterial,
    "GET_MATERIAL_FROM_REAGENT" => MaterialType::GetMaterialFromReagent,

    // Special "Hardcoded" Materials
    "AMBER" => MaterialType::Amber,
    "CORAL" => MaterialType::Coral,
    "GLASS_GREEN" => MaterialType::GlassGreen,
    "GLASS_CLEAR" => MaterialType::GlassClear,
    "GLASS_CRYSTAL" => MaterialType::GlassCrystal,
    "WATER" => MaterialType::Water,
    "POTASH" => MaterialType::Potash,
    "ASH" => MaterialType::Ash,
    "PEARLASH" => MaterialType::PearlAsh,
    "LYE" => MaterialType::Lye,
    "MUD" => MaterialType::Mud,
    "VOMIT" => MaterialType::Vomit,
    "SALT" => MaterialType::Salt,
    "FILTH_B" => MaterialType::FilthB,
    "FILTH_Y" => MaterialType::FilthY,
    "UNKNOWN_SUBSTANCE" => MaterialType::UnknownSubstance,
    "GRIME" => MaterialType::Grime,
};

pub static FUEL_TYPE_TOKENS: phf::Map<&'static str, FuelType> = phf::phf_map! {
    "COAL" => FuelType::Charcoal,
    "COKE" => FuelType::Coke,
    "NO_MATGLOSS" => FuelType::NoMaterialGloss,
};

pub static MATERIAL_USAGE_TOKENS: phf::Map<&'static str, MaterialUsage> = phf::phf_map! {
    "IMPLIES_ANIMAL_KILL" => MaterialUsage::ImpliesAnimalKill,
    "ALCOHOL_PLANT" => MaterialUsage::AlcoholPlant,
    "ALCOHOL_CREATURE" => MaterialUsage::AlcoholCreature,
    "ALCOHOL" => MaterialUsage::Alcohol,
    "CHEESE_PLANT" => MaterialUsage::CheesePlant,
    "CHEESE_CREATURE" => MaterialUsage::CheeseCreature,
    "CHEESE" => MaterialUsage::Cheese,
    "POWDER_MISC_PLANT" => MaterialUsage::PowderMiscPlant,
    "POWDER_MISC_CREATURE" => MaterialUsage::PowderMiscCreature,
    "POWDER_MISC" => MaterialUsage::PowderMisc,
    "STOCKPILE_GLOB" => MaterialUsage::StockpileGlobOrStockpileGlobSolid,
    "STOCKPILE_GLOB_SOLID" => MaterialUsage::StockpileGlobOrStockpileGlobSolid,
    "STOCKPILE_GLOB_PASTE" => MaterialUsage::StockpileGlobPaste,
    "STOCKPILE_GLOB_PRESSED" => MaterialUsage::StockpileGlobPressed,
    "STOCKPILE_PLANT_GROWTH" => MaterialUsage::StockpilePlantGrowth,
    "LIQUID_MISC_PLANT" => MaterialUsage::LiquidMiscPlant,
    "LIQUID_MISC_CREATURE" => MaterialUsage::LiquidMiscCreature,
    "LIQUID_MISC_OTHER" => MaterialUsage::LiquidMiscOther,
    "LIQUID_MISC" => MaterialUsage::LiquidMisc,
    "STRUCTURAL_PLANT_MAT" => MaterialUsage::StructuralPlantMat,
    "SEED_MAT" => MaterialUsage::SeedMat,
    "BONE" => MaterialUsage::Bone,
    "WOOD" => MaterialUsage::Wood,
    "THREAD_PLANT" => MaterialUsage::ThreadPlant,
    "TOOTH" => MaterialUsage::Tooth,
    "HORN" => MaterialUsage::Horn,
    "HAIR" => MaterialUsage::Hair,
    "PEARL" => MaterialUsage::Pearl,
    "SHELL" => MaterialUsage::Shell,
    "LEATHER" => MaterialUsage::Leather,
    "SILK" => MaterialUsage::Silk,
    "SOAP" => MaterialUsage::Soap,
    "GENERATES_MIASMA" => MaterialUsage::GeneratesMiasma,
    "MEAT" => MaterialUsage::Meat,
    "ROTS" => MaterialUsage::Rots,
    "NERVOUS_TISSUE" => MaterialUsage::NervousTissue,
    "BLOOD_MAP_DESCRIPTOR" => MaterialUsage::BloodMapDescriptor,
    "ICHOR_MAP_DESCRIPTOR" => MaterialUsage::IchorMapDescriptor,
    "GOO_MAP_DESCRIPTOR" => MaterialUsage::GooMapDescriptor,
    "SLIME_MAP_DESCRIPTOR" => MaterialUsage::SlimeMapDescriptor,
    "PUS_MAP_DESCRIPTOR" => MaterialUsage::PusMapDescriptor,
    "SWEAT_MAP_DESCRIPTOR" => MaterialUsage::SweatMapDescriptor,
    "TEARS_MAP_DESCRIPTOR" => MaterialUsage::TearsMapDescriptor,
    "SPIT_MAP_DESCRIPTOR" => MaterialUsage::SpitMapDescriptor,
    "EVAPORATES" => MaterialUsage::Evaporates,
    "ENTERS_BLOOD" => MaterialUsage::EntersBlood,
    "EDIBLE_VERMIN" => MaterialUsage::EdibleVermin,
    "EDIBLE_RAW" => MaterialUsage::EdibleRaw,
    "EDIBLE_COOKED" => MaterialUsage::EdibleCooked,
    "DO_NOT_CLEAN_GLOB" => MaterialUsage::DoNotCleanGlob,
    "NO_STONE_STOCKPILE" => MaterialUsage::NoStoneStockpile,
    "ITEMS_METAL" => MaterialUsage::ItemsMetal,
    "ITEMS_BARRED" => MaterialUsage::ItemsBarred,
    "ITEMS_SCALED" => MaterialUsage::ItemsScaled,
    "ITEMS_LEATHER" => MaterialUsage::ItemsLeather,
    "ITEMS_SOFT" => MaterialUsage::ItemsSoft,
    "ITEMS_HARD" => MaterialUsage::ItemsHard,
    "IS_STONE" => MaterialUsage::IsStone,
    "IS_CERAMIC" => MaterialUsage::IsCeramic,
    "UNDIGGABLE" => MaterialUsage::Undiggable,
    "DISPLAY_UNGLAZED" => MaterialUsage::DisplayUnglazed,
    "YARN" => MaterialUsage::Yarn,
    "STOCKPILE_THREAD_METAL" => MaterialUsage::StockpileThreadMetal,
    "IS_METAL" => MaterialUsage::IsMetal,
    "IS_GLASS" => MaterialUsage::IsGlass,
    "CRYSTAL_GLASSABLE" => MaterialUsage::CrystalGlassable,
    "ITEMS_WEAPON" => MaterialUsage::ItemsWeapon,
    "ITEMS_WEAPON_RANGED" => MaterialUsage::ItemsWeaponRanged,
    "ITEMS_ANVIL" => MaterialUsage::ItemsAnvil,
    "ITEMS_AMMO" => MaterialUsage::ItemsAmmo,
    "ITEMS_DIGGER" => MaterialUsage::ItemsDigger,
    "ITEMS_ARMOR" => MaterialUsage::ItemsArmor,
    "ITEMS_DELICATE" => MaterialUsage::ItemsDelicate,
    "ITEMS_SIEGE_ENGINE" => MaterialUsage::ItemsSiegeEngine,
    "ITEMS_QUERN" => MaterialUsage::ItemsQuern,
};

pub static MATERIAL_PROPERTY_TOKENS: phf::Map<&'static str, MaterialProperty> = phf::phf_map! {
    "USE_MATERIAL_TEMPLATE" => MaterialProperty::UseMaterialTemplate,
    "PREFIX" => MaterialProperty::Prefix,
    "STONE_NAME" => MaterialProperty::StoneName,
    "IS_GEM" => MaterialProperty::IsGem,
    "TEMP_DIET_INFO" => MaterialProperty::TempDietInfo,
    "POWDER_DYE" => MaterialProperty::PowderDye,
    "TILE" => MaterialProperty::Tile,
    "ITEM_SYMBOL" => MaterialProperty::ItemSymbol,
    "DISPLAY_COLOR" => MaterialProperty::DisplayColor,
    "BUILD_COLOR" => MaterialProperty::BuildColor,
    "TILE_COLOR" => MaterialProperty::TileColor,
    "BASIC_COLOR" => MaterialProperty::BasicColor,
    "STATE_COLOR" => MaterialProperty::StateColor,
    "STATE_NAME" => MaterialProperty::StateName,
    "STATE_ADJ" => MaterialProperty::StateAdjective,
    "STATE_NAME_ADJ" => MaterialProperty::StateNameAdjective,
    "ABSORPTION" => MaterialProperty::Absorption,
    "IMPACT_YIELD" => MaterialProperty::ImpactYield,
    "IMPACT_FRACTURE" => MaterialProperty::ImpactFracture,
    "IMPACT_STRAIN_AT_YIELD" => MaterialProperty::ImpactElasticity,
    "IMPACT_ELASTICITY" => MaterialProperty::ImpactElasticity,
    "COMPRESSIVE_YIELD" => MaterialProperty::CompressiveYield,
    "COMPRESSIVE_FRACTURE" => MaterialProperty::CompressiveFracture,
    "COMPRESSIVE_STRAIN_AT_YIELD" => MaterialProperty::CompressiveElasticity,
    "COMPRESSIVE_ELASTICITY" => MaterialProperty::CompressiveElasticity,
    "TENSILE_YIELD" => MaterialProperty::TensileYield,
    "TENSILE_FRACTURE" => MaterialProperty::TensileFracture,
    "TENSILE_STRAIN_AT_YIELD" => MaterialProperty::TensileElasticity,
    "TENSILE_ELASTICITY" => MaterialProperty::TensileElasticity,
    "TORSION_YIELD" => MaterialProperty::TorsionYield,
    "TORSION_FRACTURE" => MaterialProperty::TorsionFracture,
    "TORSION_STRAIN_AT_YIELD" => MaterialProperty::TorsionElasticity,
    "TORSION_ELASTICITY" => MaterialProperty::TorsionElasticity,
    "SHEAR_YIELD" => MaterialProperty::ShearYield,
    "SHEAR_FRACTURE" => MaterialProperty::ShearFracture,
    "SHEAR_STRAIN_AT_YIELD" => MaterialProperty::ShearElasticity,
    "SHEAR_ELASTICITY" => MaterialProperty::ShearElasticity,
    "BENDING_YIELD" => MaterialProperty::BendingYield,
    "BENDING_FRACTURE" => MaterialProperty::BendingFracture,
    "BENDING_STRAIN_AT_YIELD" => MaterialProperty::BendingElasticity,
    "BENDING_ELASTICITY" => MaterialProperty::BendingElasticity,
    "MAX_EDGE" => MaterialProperty::MaxEdge,
    "MATERIAL_VALUE" => MaterialProperty::MaterialValue,
    "MULTIPLY_VALUE" => MaterialProperty::MultiplyValue,
    "SPEC_HEAT" => MaterialProperty::SpecificHeat,
    "HEATDAM_POINT" => MaterialProperty::HeatDamagePoint,
    "COLDDAM_POINT" => MaterialProperty::ColdDamagePoint,
    "IGNITE_POINT" => MaterialProperty::IgnitionPoint,
    "MELTING_POINT" => MaterialProperty::MeltingPoint,
    "BOILING_POINT" => MaterialProperty::BoilingPoint,
    "MAT_FIXED_TEMP" => MaterialProperty::MaterialFixedTemperature,
    "IF_EXISTS_SET_HEATDAM_POINT" => MaterialProperty::IfExistsSetHeatDamagePoint,
    "IF_EXISTS_SET_COLDDAM_POINT" => MaterialProperty::IfExistsSetColdDamagePoint,
    "IF_EXISTS_SET_IGNITE_POINT" => MaterialProperty::IfExistsSetIgnitePoint,
    "IF_EXISTS_SET_MELTING_POINT" => MaterialProperty::IfExistsSetMeltingPoint,
    "IF_EXISTS_SET_BOILING_POINT" => MaterialProperty::IfExistsSetBoilingPoint,
    "IF_EXISTS_SET_MAT_FIXED_TEMP" => MaterialProperty::IfExistsSetMatFixedTemp,
    "SOLID_DENSITY" => MaterialProperty::SolidDensity,
    "LIQUID_DENSITY" => MaterialProperty::LiquidDensity,
    "MOLAR_MASS" => MaterialProperty::MolarMass,
    "EXTRACT_STORAGE" => MaterialProperty::ExtractStorage,
    "BUTCHER_SPECIAL" => MaterialProperty::ButcherSpecial,
    "MEAT_NAME" => MaterialProperty::MeatName,
    "BLOCK_NAME" => MaterialProperty::BlockName,
    "WAFERS" => MaterialProperty::Wafers,
    "MATERIAL_REACTION_PRODUCT" => MaterialProperty::MaterialReactionProduct,
    "ITEM_REACTION_PRODUCT" => MaterialProperty::ItemReactionProduct,
    "REACTION_CLASS" => MaterialProperty::ReactionClass,
    "METAL_ORE" => MaterialProperty::MetalOre,
    "THREAD_METAL" => MaterialProperty::ThreadMetal,
    "HARDENS_WITH_WATER" => MaterialProperty::HardensWithWater,
    "SOAP_LEVEL" => MaterialProperty::SoapLevel,
    "SYNDROME" => MaterialProperty::Syndrome,
    "ANTLER" => MaterialProperty::Antler,
    "CARTILAGE" => MaterialProperty::Cartilage,
    "FEATHER" => MaterialProperty::Feather,
    "HOOF" => MaterialProperty::Hoof,
    "CHITIN" => MaterialProperty::Chitin,
    "SCALE" => MaterialProperty::Scale,
    "NERVOUS_TISSUE" => MaterialProperty::NervousTissue,
    "MEAT_CATEGORY" => MaterialProperty::MeatCategory,
};