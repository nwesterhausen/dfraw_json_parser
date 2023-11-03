use super::tokens::Biome;

/// The perfect hash table for biome tokens and the Biome enum.
/// 
/// This is a mapping which takes the string of how the biome is written in the
/// raws and returns the Biome enum variant.
/// 
/// This is made static by the `phf` crate at compile time.
pub static BIOME_TOKENS: phf::Map<&'static str, Biome> = phf::phf_map! {
    "MOUNTAIN" => Biome::Mountain,
    "MOUNTAINS" => Biome::Mountains,
    "GLACIER" => Biome::Glacier,
    "TUNDRA" => Biome::Tundra,
    "SWAMP_TEMPERATE_FRESHWATER" => Biome::SwampTemperateFreshwater,
    "SWAMP_TEMPERATE_SALTWATER" => Biome::SwampTemperateSaltwater,
    "MARSH_TEMPERATE_FRESHWATER" => Biome::MarshTemperateFreshwater,
    "MARSH_TEMPERATE_SALTWATER" => Biome::MarshTemperateSaltwater,
    "SWAMP_TROPICAL_FRESHWATER" => Biome::SwampTropicalFreshwater,
    "SWAMP_TROPICAL_SALTWATER" => Biome::SwampTropicalSaltwater,
    "SWAMP_MANGROVE" => Biome::SwampMangrove,
    "MARSH_TROPICAL_FRESHWATER" => Biome::MarshTropicalFreshwater,
    "MARSH_TROPICAL_SALTWATER" => Biome::MarshTropicalSaltwater,
    "FOREST_TAIGA" => Biome::ForestTaiga,
    "TAIGA" => Biome::Taiga,
    "FOREST_TEMPERATE_CONIFER" => Biome::ForestTemperateConifer,
    "FOREST_TEMPERATE_BROADLEAF" => Biome::ForestTemperateBroadleaf,
    "FOREST_TROPICAL_CONIFER" => Biome::ForestTropicalConifer,
    "FOREST_TROPICAL_DRY_BROADLEAF" => Biome::ForestTropicalDryBroadleaf,
    "FOREST_TROPICAL_MOIST_BROADLEAF" => Biome::ForestTropicalMoistBroadleaf,
    "GRASSLAND_TEMPERATE" => Biome::GrasslandTemperate,
    "SAVANNA_TEMPERATE" => Biome::SavannaTemperate,
    "SHRUBLAND_TEMPERATE" => Biome::ShrublandTemperate,
    "GRASSLAND_TROPICAL" => Biome::GrasslandTropical,
    "SAVANNA_TROPICAL" => Biome::SavannaTropical,
    "SHRUBLAND_TROPICAL" => Biome::ShrublandTropical,
    "DESERT_BADLAND" => Biome::DesertBadland,
    "DESERT_ROCK" => Biome::DesertRock,
    "DESERT_SAND" => Biome::DesertSand,
    "OCEAN_TROPICAL" => Biome::OceanTropical,
    "OCEAN_TEMPERATE" => Biome::OceanTemperate,
    "OCEAN_ARCTIC" => Biome::OceanArctic,
    "POOL_TEMPERATE_FRESHWATER" => Biome::PoolTemperateFreshwater,
    "POOL_TEMPERATE_BRACKISHWATER" => Biome::PoolTemperateBrackishwater,
    "POOL_TEMPERATE_SALTWATER" => Biome::PoolTemperateSaltwater,
    "POOL_TROPICAL_FRESHWATER" => Biome::PoolTropicalFreshwater,
    "POOL_TROPICAL_BRACKISHWATER" => Biome::PoolTropicalBrackishwater,
    "POOL_TROPICAL_SALTWATER" => Biome::PoolTropicalSaltwater,
    "LAKE_TEMPERATE_FRESHWATER" => Biome::LakeTemperateFreshwater,
    "LAKE_TEMPERATE_BRACKISHWATER" => Biome::LakeTemperateBrackishwater,
    "LAKE_TEMPERATE_SALTWATER" => Biome::LakeTemperateSaltwater,
    "LAKE_TROPICAL_FRESHWATER" => Biome::LakeTropicalFreshwater,
    "LAKE_TROPICAL_BRACKISHWATER" => Biome::LakeTropicalBrackishwater,
    "LAKE_TROPICAL_SALTWATER" => Biome::LakeTropicalSaltwater,
    "RIVER_TEMPERATE_FRESHWATER" => Biome::RiverTemperateFreshwater,
    "RIVER_TEMPERATE_BRACKISHWATER" => Biome::RiverTemperateBrackishwater,
    "RIVER_TEMPERATE_SALTWATER" => Biome::RiverTemperateSaltwater,
    "RIVER_TROPICAL_FRESHWATER" => Biome::RiverTropicalFreshwater,
    "RIVER_TROPICAL_BRACKISHWATER" => Biome::RiverTropicalBrackishwater,
    "RIVER_TROPICAL_SALTWATER" => Biome::RiverTropicalSaltwater,
    "SUBTERRANEAN_WATER" => Biome::SubterraneanWater,
    "SUBTERRANEAN_CHASM" => Biome::SubterraneanChasm,
    "SUBTERRANEAN_LAVA" => Biome::SubterraneanLava,
    "ALL_MAIN" => Biome::AllMain,
    "ANY_LAND" => Biome::AnyLand,
    "ANY_OCEAN" => Biome::AnyOcean,
    "ANY_LAKE" => Biome::AnyLake,
    "ANY_TEMPERATE_LAKE" => Biome::AnyTemperateLake,
    "ANY_TROPICAL_LAKE" => Biome::AnyTropicalLake,
    "ANY_RIVER" => Biome::AnyRiver,
    "ANY_TEMPERATE_RIVER" => Biome::AnyTemperateRiver,
    "ANY_TROPICAL_RIVER" => Biome::AnyTropicalRiver,
    "ANY_POOL" => Biome::AnyPool,
    "NOT_FREEZING" => Biome::NotFreezing,
    "ANY_TEMPERATE" => Biome::AnyTemperate,
    "ANY_TROPICAL" => Biome::AnyTropical,
    "ANY_FOREST" => Biome::AnyForest,
    "ANY_SHRUBLAND" => Biome::AnyShrubland,
    "ANY_GRASSLAND" => Biome::AnyGrassland,
    "ANY_SAVANNA" => Biome::AnySavanna,
    "ANY_TEMPERATE_FOREST" => Biome::AnyTemperateForest,
    "ANY_TROPICAL_FOREST" => Biome::AnyTropicalForest,
    "ANY_TEMPERATE_BROADLEAF" => Biome::AnyTemperateBroadleaf,
    "ANY_TROPICAL_BROADLEAF" => Biome::AnyTropicalBroadleaf,
    "ANY_WETLAND" => Biome::AnyWetland,
    "ANY_TEMPERATE_WETLAND" => Biome::AnyTemperateWetland,
    "ANY_TROPICAL_WETLAND" => Biome::AnyTropicalWetland,
    "ANY_TROPICAL_MARSH" => Biome::AnyTropicalMarsh,
    "ANY_TEMPERATE_MARSH" => Biome::AnyTemperateMarsh,
    "ANY_TROPICAL_SWAMP" => Biome::AnyTropicalSwamp,
    "ANY_TEMPERATE_SWAMP" => Biome::AnyTemperateSwamp,
    "ANY_DESERT" => Biome::AnyDesert,
};

impl std::fmt::Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
    Biome::Mountain => write!(f, "Mountain"),
    Biome::Mountains => write!(f, "Mountains"),
    Biome::Glacier => write!(f, "Glacier"),
    Biome::Tundra => write!(f, "Tundra"),
    Biome::SwampTemperateFreshwater => write!(f, "Temperate Freshwater Swamp"),
    Biome::SwampTemperateSaltwater => write!(f, "Temperate Saltwater Swamp"),
    Biome::MarshTemperateFreshwater => write!(f, "Temperate Freshwater Marsh"),
    Biome::MarshTemperateSaltwater => write!(f, "Temperate Saltwater Marsh"),
    Biome::SwampTropicalFreshwater => write!(f, "Tropical Freshwater Swamp"),
    Biome::SwampTropicalSaltwater => write!(f, "Tropical Saltwater Swamp"),
    Biome::SwampMangrove => write!(f, "Mangrove Swamp"),
    Biome::MarshTropicalFreshwater => write!(f, "Tropical Freshwater Marsh"),
    Biome::MarshTropicalSaltwater => write!(f, "Tropical Saltwater Marsh"),
    Biome::ForestTaiga => write!(f, "Taiga Forest"),
    Biome::Taiga => write!(f, "Taiga"),
    Biome::ForestTemperateConifer => write!(f, "Temperate Coniferous Forest"),
    Biome::ForestTemperateBroadleaf => write!(f, "Temperate Broadleaf Forest"),
    Biome::ForestTropicalConifer => write!(f, "Tropical Coniferous Forest"),
    Biome::ForestTropicalDryBroadleaf => write!(f, "Tropical Dry Broadleaf Forest"),
    Biome::ForestTropicalMoistBroadleaf => write!(f, "Tropical Moist Broadleaf Forest"),
    Biome::GrasslandTemperate => write!(f, "Temperate Grassland"),
    Biome::SavannaTemperate => write!(f, "Temperate Savanna"),
    Biome::ShrublandTemperate => write!(f, "Temperate Shrubland"),
    Biome::GrasslandTropical => write!(f, "Tropical Grassland"),
    Biome::SavannaTropical => write!(f, "Tropical Savanna"),
    Biome::ShrublandTropical => write!(f, "Tropical Shrubland"),
    Biome::DesertBadland => write!(f, "Badlands"),
    Biome::DesertRock => write!(f, "Rocky Wasteland"),
    Biome::DesertSand => write!(f, "Sand Desert"),
    Biome::OceanTropical => write!(f, "Tropical Ocean"),
    Biome::OceanTemperate => write!(f, "Temperate Ocean"),
    Biome::OceanArctic => write!(f, "Arctic Ocean"),
    Biome::PoolTemperateFreshwater => write!(f, "Temperate Freshwater Pool"),
    Biome::PoolTemperateBrackishwater => write!(f, "Temperate Brackish Pool"),
    Biome::PoolTemperateSaltwater => write!(f, "Temperate Saltwater Pool"),
    Biome::PoolTropicalFreshwater => write!(f, "Tropical Freshwater Pool"),
    Biome::PoolTropicalBrackishwater => write!(f, "Tropical Brackish Pool"),
    Biome::PoolTropicalSaltwater => write!(f, "Tropical Saltwater Pool"),
    Biome::LakeTemperateFreshwater => write!(f, "Temperate Freshwater Lake"),
    Biome::LakeTemperateBrackishwater => write!(f, "Temperate Brackish Lake"),
    Biome::LakeTemperateSaltwater => write!(f, "Temperate Saltwater Lake"),
    Biome::LakeTropicalFreshwater => write!(f, "Tropical Freshwater Lake"),
    Biome::LakeTropicalBrackishwater => write!(f, "Tropical Brackish Lake"),
    Biome::LakeTropicalSaltwater => write!(f, "Tropical Saltwater Lake"),
    Biome::RiverTemperateFreshwater => write!(f, "Temperate Freshwater River"),
    Biome::RiverTemperateBrackishwater => write!(f, "Temperate Brackish River"),
    Biome::RiverTemperateSaltwater => write!(f, "Temperate Saltwater River"),
    Biome::RiverTropicalFreshwater => write!(f, "Tropical Freshwater River"),
    Biome::RiverTropicalBrackishwater => write!(f, "Tropical Brackish River"),
    Biome::RiverTropicalSaltwater => write!(f, "Tropical Saltwater River"),
    Biome::SubterraneanWater => write!(f, "Underground caverns (in water)"),
    Biome::SubterraneanChasm => write!(f, "Underground caverns (out of water)"),
    Biome::SubterraneanLava => write!(f, "Magma sea"),
    Biome::AllMain => write!(f, "All biomes excluding pools, rivers, and underground features"),
    Biome::AnyLand => write!(f, "All main biomes excluding oceans and lakes"),
    Biome::AnyOcean => write!(f, "All ocean biomes"),
    Biome::AnyLake => write!(f, "All lake biomes"),
    Biome::AnyTemperateLake => write!(f, "All temperate lake biomes"),
    Biome::AnyTropicalLake => write!(f, "All tropical lake biomes"),
    Biome::AnyRiver => write!(f, "All river biomes"),
    Biome::AnyTemperateRiver => write!(f, "All temperate river biomes"),
    Biome::AnyTropicalRiver => write!(f, "All tropical river biomes"),
    Biome::AnyPool => write!(f, "All pool biomes"),
    Biome::NotFreezing => write!(f, "All land biomes excluding Mountain, Glacier, and Tundra"),
    Biome::AnyTemperate => write!(f, "All Temperate land biomes - marshes, swamps, forests, grassland, savanna, and shrubland"),
    Biome::AnyTropical => write!(f, "All Tropical land biomes - marshes, swamps (including Mangrove), forests, grassland, savanna, and shrubland"),
    Biome::AnyForest => write!(f, "All Forest biomes (excluding Taiga)"),
    Biome::AnyShrubland => write!(f, "Temperate and Tropical Shrubland"),
    Biome::AnyGrassland => write!(f, "Temperate and Tropical Grassland"),
    Biome::AnySavanna => write!(f, "Temperate and Tropical Savanna"),
    Biome::AnyTemperateForest => write!(f, "Temperate Coniferous and Broadleaf Forests"),
    Biome::AnyTropicalForest => write!(f, "Tropical Coniferous and Dry/Moist Broadleaf Forests"),
    Biome::AnyTemperateBroadleaf => write!(f, "Temperate Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps, and Marshes"),
    Biome::AnyTropicalBroadleaf => write!(f, "Tropical Dry/Moist Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps (including Mangrove), and Marshes"),
    Biome::AnyWetland => write!(f, "All swamps and marshes"),
    Biome::AnyTemperateWetland => write!(f, "All temperate swamps and marshes"),
    Biome::AnyTropicalWetland => write!(f, "All tropical swamps and marshes"),
    Biome::AnyTropicalMarsh => write!(f, "All tropical marshes"),
    Biome::AnyTemperateMarsh => write!(f, "All temperate marshes"),
    Biome::AnyTropicalSwamp => write!(f, "All tropical swamps (including Mangrove)"),
    Biome::AnyTemperateSwamp => write!(f, "All temperate swamps"),
    Biome::AnyDesert => write!(f, "Badlands, Rocky Wasteland, and Sand Desert"),
    Biome::Unknown => write!(f, "Unknown"),
        }
    }
}
