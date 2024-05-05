use super::tokens::{Condition, GraphicType, GrowthToken, PlantGraphicTemplate, TilePageTag};

pub static CONDITION_TAGS: phf::Map<&'static str, Condition> = phf::phf_map! {
    "DEFAULT" => Condition::Default,
    "ANIMATED" => Condition::Animated,
    "CORPSE" => Condition::Corpse,
    "CHILD" => Condition::Child,
    "BABY" => Condition::Baby,
    "TRAINED_WAR" => Condition::TrainedWar,
    "TRAINED_HUNTER" => Condition::TrainedHunter,
    "LIST_ICON" => Condition::ListIcon,
    "SKELETON" => Condition::Skeleton,
    "SKELETON_WITH_SKULL" => Condition::SkeletonWithSkull,
    "ZOMBIE" => Condition::Zombie,
    "NECROMANCER" => Condition::Necromancer,
    "MALE" => Condition::Male,
    "FEMALE" => Condition::Female,
    "VAMPCURSE" => Condition::VampireCursed,
    "GHOUL" => Condition::Ghoul,
    "GHOST" => Condition::Ghost,
    "DISTURBED_DEAD" => Condition::DisturbedDead,
    "REMAINS" => Condition::Remains,
    "VERMIN" => Condition::Vermin,
    "LIGHT_VERMIN" => Condition::LightVermin,
    "HIVE" => Condition::Hive,
    "SWARM_SMALL" => Condition::SwarmSmall,
    "SWARM_MEDIUM" => Condition::SwarmMedium,
    "SWARM_LARGE" => Condition::SwarmLarge,

    "NOT_ARTIFACT" => Condition::NotArtifact,
    "IS_CRAFTED_ARTIFACT" => Condition::CraftedArtifact,

    "SHRUB" => Condition::Shrub,
    "PICKED" => Condition::Picked,
    "SEED" => Condition::Seed,
    "CROP" => Condition::Crop,
    "CROP_SPROUT" => Condition::CropSprout,
    "CROP_L" => Condition::CropL,
    "CROP_M" => Condition::CropM,
    "CROP_R" => Condition::CropR,
    "SHRUB_DEAD" => Condition::ShrubDead,
    "SAPLING" => Condition::Sapling,

    "CONDITION_NOT_CHILD" => Condition::NotChild,
    "CONDITION_CLASS" => Condition::Class,
    "CONDITION_CHILD" => Condition::Child,
    "CONDITION_BABY" => Condition::Baby,
    "CONDITION_HAUL_COUNT_MIN" => Condition::HaulCountMin,
    "CONDITION_HAUL_COUNT_MAX" => Condition::HaulCountMax,
    "CONDITION_ITEM_WORN" => Condition::ItemWorn,
    "CONDITION_PROFESSION_CATEGORY" => Condition::ProfessionCategory,
    "CONDITION_SYN_CLASS" => Condition::SyndromeClass,
    "CONDITION_CASTE" => Condition::Caste,
    "CONDITION_TISSUE_LAYER" => Condition::TissueLayer,
    "CONDITION_MATERIAL_FLAG" => Condition::MaterialFlag,
    "CONDITION_MATERIAL_TYPE" => Condition::MaterialType,
    "CONDITION_DYE" => Condition::Dye,
    "CONDITION_DYED" => Condition::Dye,
    "CONDITION_NOT_DYED" => Condition::NotDyed,
    "SHUT_OFF_IF_ITEM_PRESENT" => Condition::ShutOffIfItemPresent,
    "CONDITION_RANDOM_PART_INDEX" => Condition::RandomPartIndex,
    "CONDITION_GHOST" => Condition::Ghost,
    "TISSUE_MAY_HAVE_COLOR" => Condition::TissueMayHaveColor,
    "TISSUE_MIN_LENGTH" => Condition::TissueMinLength,
    "TISSUE_MAX_LENGTH" => Condition::TissueMaxLength,
    "TISSUE_MAY_HAVE_SHAPING" => Condition::TissueMayHaveShaping,
    "TISSUE_NOT_SHAPED" => Condition::TissueNotShaped,
    "TISSUE_SWAP" => Condition::TissueSwap,
    "TISSUE_MIN_CURLY" => Condition::TissueMinCurly,
    "TISSUE_MAX_CURLY" => Condition::TissueMaxCurly,
    // Generic condition (or something)
    "CONDITION" => Condition::Condition,

    // Professions...
    "HAMMERMAN" => Condition::Hammerman,
    "MASTER_HAMMERMAN" => Condition::MasterHammerman,
    "SPEARMAN" => Condition::Spearman,
    "MASTER_SPEARMAN" => Condition::MasterSpearman,
    "WRESTLER" => Condition::Wrestler,
    "MASTER_WRESTLER" => Condition::MasterWrestler,
    "AXEMAN" => Condition::Axeman,
    "MASTER_AXEMAN" => Condition::MasterAxeman,
    "SWORDSMAN" => Condition::Swordsman,
    "MASTER_SWORDSMAN" => Condition::MasterSwordsman,
    "MACEMAN" => Condition::Maceman,
    "MASTER_MACEMAN" => Condition::MasterMaceman,
    "PIKEMAN" => Condition::Pikeman,
    "MASTER_PIKEMAN" => Condition::MasterPikeman,
    "RECRUIT" => Condition::Recruit,
    "THIEF" => Condition::Thief,
    "MASTER_THIEF" => Condition::MasterThief,
    "LASHER" => Condition::Lasher,
    "MASTER_LASHER" => Condition::MasterLasher,
    "MONSTER_SLAYER" => Condition::MonsterSlayer,
    "CROSSBOWMAN" => Condition::Crossbowman,
    "MASTER_CROSSBOWMAN" => Condition::MasterCrossbowman,
    "BOWMAN" => Condition::Bowman,
    "MASTER_BOWMAN" => Condition::MasterBowman,
    "BLOWGUNMAN" => Condition::Blowgunman,
    "MASTER_BLOWGUNMAN" => Condition::MasterBlowgunman,
    "BEAST_HUNTER" => Condition::BeastHunter,
    "SCOUT" => Condition::Scout,
    "RANGER" => Condition::Ranger,
    "HUNTER" => Condition::Hunter,
    "SAGE" => Condition::Sage,
    "SCHOLAR" => Condition::Scholar,
    "PHILOSOPHER" => Condition::Philosopher,
    "MATHEMATICIAN" => Condition::Mathematician,
    "HISTORIAN" => Condition::Historian,
    "ASTRONOMER" => Condition::Astronomer,
    "NATURALIST" => Condition::Naturalist,
    "CHEMIST" => Condition::Chemist,
    "GEOGRAPHER" => Condition::Geographer,
    "SCRIBE" => Condition::Scribe,
    "BOOKBINDER" => Condition::Bookbinder,
    "PERFORMER" => Condition::Performer,
    "POET" => Condition::Poet,
    "BARD" => Condition::Bard,
    "DANCER" => Condition::Dancer,
};

pub static CUSTOM_GRAPHIC_TAGS: phf::Map<&'static str, GraphicType> = phf::phf_map! {
    // [CUSTOM_EDGING:1] can be from 1 to 32, lower number is printed with higher priority,
    // all win out over regular grass
    "CUSTOM_EDGING" => GraphicType::CustomEdging,
    // [CUSTOM_RAMP:1] can be from 1 to 32, uses the plant's GRASS_1 image
    "CUSTOM_RAMP" => GraphicType::CustomRamp,
    // [CUSTOM_EDGE_W:CAVERN_GRASS:4:0]
    "CUSTOM_EDGE_W" => GraphicType::CustomEdgeW,
    // [CUSTOM_EDGE_E:CAVERN_GRASS:5:0]
    "CUSTOM_EDGE_E" => GraphicType::CustomEdgeE,
    // [CUSTOM_EDGE_S:CAVERN_GRASS:6:0]
    "CUSTOM_EDGE_S" => GraphicType::CustomEdgeS,
    // [CUSTOM_EDGE_N:CAVERN_GRASS:7:0]
    "CUSTOM_EDGE_N" => GraphicType::CustomEdgeN,
    // [CUSTOM_EDGE_NW:CAVERN_GRASS:8:0]
    "CUSTOM_EDGE_NW" => GraphicType::CustomEdgeNW,
    // [CUSTOM_EDGE_NE:CAVERN_GRASS:9:0]
    "CUSTOM_EDGE_NE" => GraphicType::CustomEdgeNE,
    // [CUSTOM_EDGE_SW:CAVERN_GRASS:10:0]
    "CUSTOM_EDGE_SW" => GraphicType::CustomEdgeSW,
    // [CUSTOM_EDGE_SE:CAVERN_GRASS:11:0]
    "CUSTOM_EDGE_SE" => GraphicType::CustomEdgeSE,
};

pub static GRAPHIC_TYPE_TAGS: phf::Map<&'static str, GraphicType> = phf::phf_map! {
    "CREATURE_GRAPHICS" => GraphicType::Creature,
    "CREATURE_CASTE_GRAPHICS" => GraphicType::CreatureCaste,
    "TILE_GRAPHICS" => GraphicType::Tile,
    "PLANT_GRAPHICS" => GraphicType::Plant,

    // "Conditions" which need to point to plant graphics
    "SHRUB" => GraphicType::Plant,
    "SHRUB_DEAD" => GraphicType::Plant,
    "PICKED" => GraphicType::Plant,
    "SEED" => GraphicType::Plant,
    "SAPLING" => GraphicType::Plant,
    "CROP" => GraphicType::Plant,
    "CROP_SPROUT" => GraphicType::Plant,
    "CROP_L" => GraphicType::Plant,
    "CROP_M" => GraphicType::Plant,
    "CROP_R" => GraphicType::Plant,

    // Interface graphics
    // [CUSTOM_WORKSHOP_GRAPHICS:SCREW_PRESS]
    "CUSTOM_WORKSHOP_GRAPHICS" => GraphicType::CustomWorkshop,
    // [LIST_ICON:BUILDING_ICONS_TWEAKED:1:12]
    "LIST_ICON" => GraphicType::ListIcon,

    // Plant Graphics
    // [SOIL_BACKGROUND]
    "SOIL_BACKGROUND" => GraphicType::SoilBackground,
    // [GRASS_1:GRASS:0:0]
    "GRASS_1" => GraphicType::Grass1,
    // [GRASS_2:GRASS:1:0]
    "GRASS_2" => GraphicType::Grass2,
    // [GRASS_3:GRASS:2:0]
    "GRASS_3" => GraphicType::Grass3,
    // [GRASS_4:GRASS:3:0]
    "GRASS_4" => GraphicType::Grass4,

    // All other graphic types from the vanilla raws
    // [ADD_TOOL_GRAPHICS:ITEM_TOOL_LARGE_POT]
    "ADD_TOOL_GRAPHICS" => GraphicType::AddTool,
    // [AMMO_GRAPHICS:ITEM_AMMO_BOLTS]
    "AMMO_GRAPHICS" => GraphicType::Ammo,
    // [AMMO_GRAPHICS_STRAIGHT_DEFAULT:ITEM_AMMO:1:0]
    "AMMO_GRAPHICS_STRAIGHT_DEFAULT" => GraphicType::AmmoStraightDefault,
    // [AMMO_GRAPHICS_STRAIGHT_WOOD:ITEM_AMMO:0:0]
    "AMMO_GRAPHICS_STRAIGHT_WOOD" => GraphicType::AmmoStraightWood,
    // [AMMO_GRAPHICS_DIAGONAL_DEFAULT:ITEM_AMMO:1:1]
    "AMMO_GRAPHICS_DIAGONAL_DEFAULT" => GraphicType::AmmoDiagonalDefault,
    // [AMMO_GRAPHICS_DIAGONAL_WOOD:ITEM_AMMO:0:1]
    "AMMO_GRAPHICS_DIAGONAL_WOOD" => GraphicType::AmmoDiagonalWood,
    // [ARMOR_GRAPHICS:ITEMS3:1:15:ITEM_ARMOR_BREASTPLATE]
    "ARMOR_GRAPHICS" => GraphicType::Armor,
    "FOOD_GRAPHICS" => GraphicType::Food,
    // [GLOVES_GRAPHICS:ITEMS3:1:21:ITEM_GLOVES_GAUNTLETS]
    "GLOVES_GRAPHICS" => GraphicType::Gloves,
    // [HELM_GRAPHICS:ITEMS3:2:15:ITEM_HELM_HELM]
    "HELM_GRAPHICS" => GraphicType::Helm,
    // [PANTS_GRAPHICS:ITEMS3:0:17:ITEM_PANTS_PANTS]
    "PANTS_GRAPHICS" => GraphicType::Pants,
    // [ROUGH_GEM_GRAPHICS:BOULDERS:0:8:GLASS_GREEN]
    "ROUGH_GEM_GRAPHICS" => GraphicType::RoughGem,
    // [SHAPE_GRAPHICS_LARGE_GEM:BAGUETTE_CUT_GEM:GEMS:1:0]
    "SHAPE_GRAPHICS_LARGE_GEM" => GraphicType::ShapeLargeGem,
    // [SHAPE_GRAPHICS_SMALL_GEM:BAGUETTE_CUT_GEM:SMALLGEMS:0:0]
    "SHAPE_GRAPHICS_SMALL_GEM" => GraphicType::ShapeSmallGem,
    // [SHIELD_GRAPHICS:ITEMS3:3:16:ITEM_SHIELD_SHIELD]
    "SHIELD_GRAPHICS" => GraphicType::Shield,
    // [SHIELD_GRAPHICS_WOODEN:ITEMS3:3:14:ITEM_SHIELD_SHIELD]
    "SHIELD_GRAPHICS_WOODEN" => GraphicType::ShieldWooden,
    // [SHOES_GRAPHICS:ITEMS3:3:18:ITEM_SHOES_SHOES]
    "SHOES_GRAPHICS" => GraphicType::Shoes,
    // [SHOES_GRAPHICS_METAL:ITEMS3:3:22:ITEM_SHOES_BOOTS]
    "SHOES_GRAPHICS_METAL" => GraphicType::ShoesMetal,
    // [SIEGEAMMO_GRAPHICS:ITEM_SIEGEAMMO_BALLISTA]
    "SIEGEAMMO_GRAPHICS" => GraphicType::SiegeAmmo,
    // [SIEGEAMMO_GRAPHICS_STRAIGHT_DEFAULT:BALLISTA_ARROW:1:0]
    "SIEGEAMMO_GRAPHICS_STRAIGHT_DEFAULT" => GraphicType::SiegeAmmoStraightDefault,
    // [SIEGEAMMO_GRAPHICS_STRAIGHT_WOOD:BALLISTA_ARROW:0:0]
    "SIEGEAMMO_GRAPHICS_STRAIGHT_WOOD" => GraphicType::SiegeAmmoStraightWood,
    // [SIEGEAMMO_GRAPHICS_DIAGONAL_DEFAULT:BALLISTA_ARROW:1:1]
    "SIEGEAMMO_GRAPHICS_DIAGONAL_DEFAULT" => GraphicType::SiegeAmmoDiagonalDefault,
    // [SIEGEAMMO_GRAPHICS_DIAGONAL_WOOD:BALLISTA_ARROW:0:1]
    "SIEGEAMMO_GRAPHICS_DIAGONAL_WOOD" => GraphicType::SiegeAmmoDiagonalWood,
    // [TOOL_GRAPHICS:TOOLS:0:0:ITEM_TOOL_CAULDRON]
    "TOOL_GRAPHICS" => GraphicType::Tool,
    // [TOOL_GRAPHICS_WOOD:1:TOOLS:0:11]
    "TOOL_GRAPHICS_WOOD" => GraphicType::ToolWood,
    // [TOOL_GRAPHICS_STONE:1:TOOLS:1:11]
    "TOOL_GRAPHICS_STONE" => GraphicType::ToolStone,
    // [TOOL_GRAPHICS_METAL:1:TOOLS:2:11]
    "TOOL_GRAPHICS_METAL" => GraphicType::ToolMetal,
    // [TOOL_GRAPHICS_HIVE_BLD:CONTAINERS:0:5]
    "TOOL_GRAPHICS_HIVE_BLD" => GraphicType::ToolHiveBuilding,
    // [TOOL_GRAPHICS_GLASS:1:ITEM_BOOKCASE:0:3]
    "TOOL_GRAPHICS_GLASS" => GraphicType::ToolGlass,
    // [TOOL_GRAPHICS_SHAPE:PLATONIC_CUBE:TOOLS:1:29]
    "TOOL_GRAPHICS_SHAPE" => GraphicType::ToolShape,
    // [TOOL_GRAPHICS_GLASS_VARIANT:3:ITEM_BOOKCASE:9:3]
    "TOOL_GRAPHICS_GLASS_VARIANT" => GraphicType::ToolGlassVariant,
    // [TOOL_GRAPHICS_METAL_VARIANT:3:ITEM_BOOKCASE:9:2]
    "TOOL_GRAPHICS_METAL_VARIANT" => GraphicType::ToolMetalVariant,
    // [TOOL_GRAPHICS_STONE_VARIANT:3:ITEM_BOOKCASE:9:1]
    "TOOL_GRAPHICS_STONE_VARIANT" => GraphicType::ToolStoneVariant,
    // [TOOL_GRAPHICS_WOOD_VARIANT:3:ITEM_BOOKCASE:9:0]
    "TOOL_GRAPHICS_WOOD_VARIANT" => GraphicType::ToolWoodVariant,
    // [TOOL_GRAPHICS_MUD:ITEM_BOOKCASE:12:3]
    "TOOL_GRAPHICS_MUD" => GraphicType::ToolMud,
    // [TOOL_GRAPHICS_WATER:ITEM_BOOKCASE:12:2]
    "TOOL_GRAPHICS_WATER" => GraphicType::ToolWater,
    // [TOOL_GRAPHICS_VOMIT:ITEM_BOOKCASE:12:1]
    "TOOL_GRAPHICS_VOMIT" => GraphicType::ToolVomit,
    // [TOOL_GRAPHICS_BLOOD:ITEM_BOOKCASE:12:0]
    "TOOL_GRAPHICS_BLOOD" => GraphicType::ToolBlood,
    // [TOOL_GRAPHICS_DAMAGE:3:ITEM_BOOKCASE:11:0]
    "TOOL_GRAPHICS_DAMAGE" => GraphicType::ToolDamage,
    // [TOOL_GRAPHICS_BANDS:ITEM_BOOKCASE:11:3]
    "TOOL_GRAPHICS_BANDS" => GraphicType::ToolBands,
    // [TOOL_GRAPHICS_ENGRAVING:ITEM_BOOKCASE:10:3]
    "TOOL_GRAPHICS_ENGRAVING" => GraphicType::ToolEngraving,
    // [TOOL_GRAPHICS_STUDS:ITEM_BOOKCASE:10:2]
    "TOOL_GRAPHICS_STUDS" => GraphicType::ToolStuds,
    // [TOOL_GRAPHICS_RINGS:ITEM_BOOKCASE:10:1]
    "TOOL_GRAPHICS_RINGS" => GraphicType::ToolRings,
    // [TOOL_GRAPHICS_SPIKES:ITEM_BOOKCASE:10:0]
    "TOOL_GRAPHICS_SPIKES" => GraphicType::ToolSpikes,
    // [TOY_GRAPHICS:ITEM_TOY:0:4:ITEM_TOY_MINIFORGE:WOOD]
    "TOY_GRAPHICS" => GraphicType::Toy,
    // [TRAPCOMP_GRAPHICS:ITEM_TRAPCOMP:0:0:ITEM_TRAPCOMP_GIANTAXEBLADE]
    "TRAPCOMP_GRAPHICS" => GraphicType::TrapComponent,
    // [TRAPCOMP_GRAPHICS_WEAPON_TRAP:TRAPS_WEAPON:3:1]
    "TRAPCOMP_GRAPHICS_WEAPON_TRAP" => GraphicType::TrapComponentWeaponTrap,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_1T:UPRIGHT_WEAPONS:0:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_1T" => GraphicType::TrapComponentUpright1T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_2T:UPRIGHT_WEAPONS:1:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_2T" => GraphicType::TrapComponentUpright2T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_3T:UPRIGHT_WEAPONS:2:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_3T" => GraphicType::TrapComponentUpright3T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_4T:UPRIGHT_WEAPONS:3:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_4T" => GraphicType::TrapComponentUpright4T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_5T:UPRIGHT_WEAPONS:4:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_5T" => GraphicType::TrapComponentUpright5T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_6T:UPRIGHT_WEAPONS:5:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_6T" => GraphicType::TrapComponentUpright6T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_7T:UPRIGHT_WEAPONS:6:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_7T" => GraphicType::TrapComponentUpright7T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_8T:UPRIGHT_WEAPONS:7:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_8T" => GraphicType::TrapComponentUpright8T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_9T:UPRIGHT_WEAPONS:8:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_9T" => GraphicType::TrapComponentUpright9T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_10T:UPRIGHT_WEAPONS:9:1]
    "TRAPCOMP_GRAPHICS_UPRIGHT_10T" => GraphicType::TrapComponentUpright10T,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_1B:UPRIGHT_WEAPONS:0:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_1B" => GraphicType::TrapComponentUpright1B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_2B:UPRIGHT_WEAPONS:1:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_2B" => GraphicType::TrapComponentUpright2B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_3B:UPRIGHT_WEAPONS:2:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_3B" => GraphicType::TrapComponentUpright3B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_4B:UPRIGHT_WEAPONS:3:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_4B" => GraphicType::TrapComponentUpright4B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_5B:UPRIGHT_WEAPONS:4:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_5B" => GraphicType::TrapComponentUpright5B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_6B:UPRIGHT_WEAPONS:5:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_6B" => GraphicType::TrapComponentUpright6B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_7B:UPRIGHT_WEAPONS:6:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_7B" => GraphicType::TrapComponentUpright7B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_8B:UPRIGHT_WEAPONS:7:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_8B" => GraphicType::TrapComponentUpright8B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_9B:UPRIGHT_WEAPONS:8:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_9B" => GraphicType::TrapComponentUpright9B,
    // [TRAPCOMP_GRAPHICS_UPRIGHT_10B:UPRIGHT_WEAPONS:9:2]
    "TRAPCOMP_GRAPHICS_UPRIGHT_10B" => GraphicType::TrapComponentUpright10B,
    // [WEAPON_GRAPHICS:ITEM_WEAPON_WHIP]
    "WEAPON_GRAPHICS" => GraphicType::Weapon,
    // [WEAPON_GRAPHICS_DEFAULT:WEAPONS:0:0]
    "WEAPON_GRAPHICS_DEFAULT" => GraphicType::WeaponDefault,
    // [WEAPON_GRAPHICS_WOOD:WEAPONS:2:0]
    "WEAPON_GRAPHICS_WOOD" => GraphicType::WeaponWood,
    // [WEAPON_GRAPHICS_WOOD_GROWN:WEAPONS:1:0]
    "WEAPON_GRAPHICS_WOOD_GROWN" => GraphicType::WeaponWoodGrown,
    // [WEAPON_GRAPHICS_MATERIAL:WEAPONS:0:0]
    "WEAPON_GRAPHICS_MATERIAL" => GraphicType::WeaponMaterial,
    // [WEAPON_GRAPHICS_WEAPON_TRAP:TRAPS_WEAPON:7:3]
    "WEAPON_GRAPHICS_WEAPON_TRAP" => GraphicType::WeaponTrap,
    // [WEAPON_GRAPHICS_UPRIGHT_1T:UPRIGHT_WEAPONS:0:3]
    "WEAPON_GRAPHICS_UPRIGHT_1T" => GraphicType::WeaponUpright1T,
    // [WEAPON_GRAPHICS_UPRIGHT_2T:UPRIGHT_WEAPONS:1:3]
    "WEAPON_GRAPHICS_UPRIGHT_2T" => GraphicType::WeaponUpright2T,
    // [WEAPON_GRAPHICS_UPRIGHT_3T:UPRIGHT_WEAPONS:2:3]
    "WEAPON_GRAPHICS_UPRIGHT_3T" => GraphicType::WeaponUpright3T,
    // [WEAPON_GRAPHICS_UPRIGHT_4T:UPRIGHT_WEAPONS:3:3]
    "WEAPON_GRAPHICS_UPRIGHT_4T" => GraphicType::WeaponUpright4T,
    // [WEAPON_GRAPHICS_UPRIGHT_5T:UPRIGHT_WEAPONS:4:3]
    "WEAPON_GRAPHICS_UPRIGHT_5T" => GraphicType::WeaponUpright5T,
    // [WEAPON_GRAPHICS_UPRIGHT_6T:UPRIGHT_WEAPONS:5:3]
    "WEAPON_GRAPHICS_UPRIGHT_6T" => GraphicType::WeaponUpright6T,
    // [WEAPON_GRAPHICS_UPRIGHT_7T:UPRIGHT_WEAPONS:6:3]
    "WEAPON_GRAPHICS_UPRIGHT_7T" => GraphicType::WeaponUpright7T,
    // [WEAPON_GRAPHICS_UPRIGHT_8T:UPRIGHT_WEAPONS:7:3]
    "WEAPON_GRAPHICS_UPRIGHT_8T" => GraphicType::WeaponUpright8T,
    // [WEAPON_GRAPHICS_UPRIGHT_9T:UPRIGHT_WEAPONS:8:3]
    "WEAPON_GRAPHICS_UPRIGHT_9T" => GraphicType::WeaponUpright9T,
    // [WEAPON_GRAPHICS_UPRIGHT_10T:UPRIGHT_WEAPONS:9:3]
    "WEAPON_GRAPHICS_UPRIGHT_10T" => GraphicType::WeaponUpright10T,
    // [WEAPON_GRAPHICS_UPRIGHT_1B:UPRIGHT_WEAPONS:0:4]
    "WEAPON_GRAPHICS_UPRIGHT_1B" => GraphicType::WeaponUpright1B,
    // [WEAPON_GRAPHICS_UPRIGHT_2B:UPRIGHT_WEAPONS:1:4]
    "WEAPON_GRAPHICS_UPRIGHT_2B" => GraphicType::WeaponUpright2B,
    // [WEAPON_GRAPHICS_UPRIGHT_3B:UPRIGHT_WEAPONS:2:4]
    "WEAPON_GRAPHICS_UPRIGHT_3B" => GraphicType::WeaponUpright3B,
    // [WEAPON_GRAPHICS_UPRIGHT_4B:UPRIGHT_WEAPONS:3:4]
    "WEAPON_GRAPHICS_UPRIGHT_4B" => GraphicType::WeaponUpright4B,
    // [WEAPON_GRAPHICS_UPRIGHT_5B:UPRIGHT_WEAPONS:4:4]
    "WEAPON_GRAPHICS_UPRIGHT_5B" => GraphicType::WeaponUpright5B,
    // [WEAPON_GRAPHICS_UPRIGHT_6B:UPRIGHT_WEAPONS:5:4]
    "WEAPON_GRAPHICS_UPRIGHT_6B" => GraphicType::WeaponUpright6B,
    // [WEAPON_GRAPHICS_UPRIGHT_7B:UPRIGHT_WEAPONS:6:4]
    "WEAPON_GRAPHICS_UPRIGHT_7B" => GraphicType::WeaponUpright7B,
    // [WEAPON_GRAPHICS_UPRIGHT_8B:UPRIGHT_WEAPONS:7:4]
    "WEAPON_GRAPHICS_UPRIGHT_8B" => GraphicType::WeaponUpright8B,
    // [WEAPON_GRAPHICS_UPRIGHT_9B:UPRIGHT_WEAPONS:8:4]
    "WEAPON_GRAPHICS_UPRIGHT_9B" => GraphicType::WeaponUpright9B,
    // [WEAPON_GRAPHICS_UPRIGHT_10B:UPRIGHT_WEAPONS:9:4]
    "WEAPON_GRAPHICS_UPRIGHT_10B" => GraphicType::WeaponUpright10B,

    // Creature Graphics
    "STATUE_CREATURE_GRAPHICS" => GraphicType::StatueCreature,
    "STATUE_CREATURE_CASTE_GRAPHICS" => GraphicType::StatueCreatureCaste,
    "STATUES_SURFACE_GIANT" => GraphicType::StatuesSurfaceGiant,

};

pub static GROWTH_TAGS: phf::Map<&'static str, GrowthToken> = phf::phf_map! {
    "GROWTH_FRUIT" => GrowthToken::Fruit,
    // [GROWTH_1:GRASS_FLOWERS:0:1]
    "GROWTH_1" => GrowthToken::Growth1,
    // [GROWTH_2:GRASS_FLOWERS:1:1]
    "GROWTH_2" => GrowthToken::Growth2,
    // [GROWTH_3:GRASS_FLOWERS:2:1]
    "GROWTH_3" => GrowthToken::Growth3,
    // [GROWTH_4:GRASS_FLOWERS:3:1]
    "GROWTH_4" => GrowthToken::Growth4,
};

pub static TILE_PAGE_TAGS: phf::Map<&'static str, TilePageTag> = phf::phf_map! {
    "TILE_DIM" => TilePageTag::TileDim,
    "PAGE_DIM_PIXELS" => TilePageTag::PageDim,
    "PAGE_DIM" => TilePageTag::PageDim,
    "FILE" => TilePageTag::File,
};

pub static PLANT_GRAPHIC_TEMPLATES: phf::Map<&'static str, PlantGraphicTemplate> = phf::phf_map! {
    "STANDARD_LEAVES" => PlantGraphicTemplate::StandardLeaves,
    "STANDARD_FLOWERS_1" => PlantGraphicTemplate::StandardFlowers1,
    "STANDARD_FRUIT_1" => PlantGraphicTemplate::StandardFruit1,
    "STANDARD_FLOWERS_2" => PlantGraphicTemplate::StandardFlowers2,
    "STANDARD_FRUIT_2" => PlantGraphicTemplate::StandardFruit2,
    "STANDARD_FLOWERS_3" => PlantGraphicTemplate::StandardFlowers3,
    "STANDARD_FRUIT_3" => PlantGraphicTemplate::StandardFruit3,
    "STANDARD_FLOWERS_4" => PlantGraphicTemplate::StandardFlowers4,
    "STANDARD_FRUIT_4" => PlantGraphicTemplate::StandardFruit4,
};
