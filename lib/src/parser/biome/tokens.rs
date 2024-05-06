use serde::{Deserialize, Serialize};

/// An enum representing a biome.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum Biome {
    /// A mountain biome.
    Mountain,
    /// A mountainous biome
    Mountains,
    /// A glacier biome.
    Glacier,
    /// A tundra biome.
    Tundra,
    /// A temperate freshwater swamp
    SwampTemperateFreshwater,
    /// A temperate saltwater swamp
    SwampTemperateSaltwater,
    /// A temperate freshwater marsh
    MarshTemperateFreshwater,
    /// A temperate saltwater marsh
    MarshTemperateSaltwater,
    /// A tropical freshwater swamp
    SwampTropicalFreshwater,
    /// A tropical saltwater swamp
    SwampTropicalSaltwater,
    /// A mangrove swamp
    SwampMangrove,
    /// A tropical freshwater marsh
    MarshTropicalFreshwater,
    /// A tropical saltwater marsh
    MarshTropicalSaltwater,
    /// A taiga forest
    ForestTaiga,
    /// A taiga
    Taiga,
    /// A temperate conifer forest
    ForestTemperateConifer,
    /// A temperate broadleaf forest
    ForestTemperateBroadleaf,
    /// A tropical conifer forest
    ForestTropicalConifer,
    /// A tropical broadleaf forest
    ForestTropicalDryBroadleaf,
    /// A tropical moist broadleaf forest
    ForestTropicalMoistBroadleaf,
    /// A temperate grassland
    GrasslandTemperate,
    /// A temperate savanna
    SavannaTemperate,
    /// A temperate shrubland
    ShrublandTemperate,
    /// A tropical grassland
    GrasslandTropical,
    /// A tropical savanna
    SavannaTropical,
    /// A tropical shrubland
    ShrublandTropical,
    /// A badland desert
    DesertBadland,
    /// A rocky desert
    DesertRock,
    /// A sandy desert
    DesertSand,
    /// A tropical ocean
    OceanTropical,
    /// A temperate ocean
    OceanTemperate,
    /// An arctic ocean
    OceanArctic,
    /// A temperate freshwater pool
    PoolTemperateFreshwater,
    /// A temperate brackishwater pool
    PoolTemperateBrackishwater,
    /// A temperate saltwater pool
    PoolTemperateSaltwater,
    /// A tropical freshwater pool
    PoolTropicalFreshwater,
    /// A tropical brackishwater pool
    PoolTropicalBrackishwater,
    /// A tropical saltwater pool
    PoolTropicalSaltwater,
    /// A temperate freshwater lake
    LakeTemperateFreshwater,
    /// A temperate brackishwater lake
    LakeTemperateBrackishwater,
    /// A temperate saltwater lake
    LakeTemperateSaltwater,
    /// A tropical freshwater lake
    LakeTropicalFreshwater,
    /// A tropical brackishwater lake
    LakeTropicalBrackishwater,
    /// A tropical saltwater lake
    LakeTropicalSaltwater,
    /// A temperate freshwater river
    RiverTemperateFreshwater,
    /// A temperate brackishwater river
    RiverTemperateBrackishwater,
    /// A temperate saltwater river
    RiverTemperateSaltwater,
    /// A tropical freshwater river
    RiverTropicalFreshwater,
    /// A tropical brackishwater river
    RiverTropicalBrackishwater,
    /// A tropical saltwater river
    RiverTropicalSaltwater,
    /// A subterranean freshwater source
    SubterraneanWater,
    /// A subterranean chasm
    SubterraneanChasm,
    /// A subterranean magma pool
    SubterraneanLava,
    /// All the main biomes
    AllMain,
    /// Any land biome
    AnyLand,
    /// Any ocean biome
    AnyOcean,
    /// Any lake biome
    AnyLake,
    /// Any temperate lake biome
    AnyTemperateLake,
    /// Any tropical lake biome
    AnyTropicalLake,
    /// Any river
    AnyRiver,
    /// Any temperate river
    AnyTemperateRiver,
    /// Any tropical river
    AnyTropicalRiver,
    /// Any pool
    AnyPool,
    /// Any non-freezing biome
    NotFreezing,
    /// Any temperate biome
    AnyTemperate,
    /// Any tropical biome
    AnyTropical,
    /// Any forest biome
    AnyForest,
    /// Any shrubland biome
    AnyShrubland,
    /// Any grassland biome
    AnyGrassland,
    /// Any savanna biome
    AnySavanna,
    /// Any temperate forest biome
    AnyTemperateForest,
    /// Any tropical forest biome
    AnyTropicalForest,
    /// Any temperate broadleaf forest biome
    AnyTemperateBroadleaf,
    /// Any tropical broadleaf forest biome
    AnyTropicalBroadleaf,
    /// Any wetland biome
    AnyWetland,
    /// Any temperate wetland biome
    AnyTemperateWetland,
    /// Any tropical wetland biome
    AnyTropicalWetland,
    /// Any tropical marsh biome
    AnyTropicalMarsh,
    /// Any temperate marsh biome
    AnyTemperateMarsh,
    /// Any tropical swamp biome
    AnyTropicalSwamp,
    /// Any temperate swamp biome
    AnyTemperateSwamp,
    /// Any desert biome
    AnyDesert,
    /// An unknown token
    #[default]
    Unknown,
}
