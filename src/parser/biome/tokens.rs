use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
/// The Biome enum contains all the possible biomes in Dwarf Fortress.
pub enum Biome {
    Mountain,
    Mountains,
    Glacier,
    Tundra,
    SwampTemperateFreshwater,
    SwampTemperateSaltwater,
    MarshTemperateFreshwater,
    MarshTemperateSaltwater,
    SwampTropicalFreshwater,
    SwampTropicalSaltwater,
    SwampMangrove,
    MarshTropicalFreshwater,
    MarshTropicalSaltwater,
    ForestTaiga,
    Taiga,
    ForestTemperateConifer,
    ForestTemperateBroadleaf,
    ForestTropicalConifer,
    ForestTropicalDryBroadleaf,
    ForestTropicalMoistBroadleaf,
    GrasslandTemperate,
    SavannaTemperate,
    ShrublandTemperate,
    GrasslandTropical,
    SavannaTropical,
    ShrublandTropical,
    DesertBadland,
    DesertRock,
    DesertSand,
    OceanTropical,
    OceanTemperate,
    OceanArctic,
    PoolTemperateFreshwater,
    PoolTemperateBrackishwater,
    PoolTemperateSaltwater,
    PoolTropicalFreshwater,
    PoolTropicalBrackishwater,
    PoolTropicalSaltwater,
    LakeTemperateFreshwater,
    LakeTemperateBrackishwater,
    LakeTemperateSaltwater,
    LakeTropicalFreshwater,
    LakeTropicalBrackishwater,
    LakeTropicalSaltwater,
    RiverTemperateFreshwater,
    RiverTemperateBrackishwater,
    RiverTemperateSaltwater,
    RiverTropicalFreshwater,
    RiverTropicalBrackishwater,
    RiverTropicalSaltwater,
    SubterraneanWater,
    SubterraneanChasm,
    SubterraneanLava,
    AllMain,
    AnyLand,
    AnyOcean,
    AnyLake,
    AnyTemperateLake,
    AnyTropicalLake,
    AnyRiver,
    AnyTemperateRiver,
    AnyTropicalRiver,
    AnyPool,
    NotFreezing,
    AnyTemperate,
    AnyTropical,
    AnyForest,
    AnyShrubland,
    AnyGrassland,
    AnySavanna,
    AnyTemperateForest,
    AnyTropicalForest,
    AnyTemperateBroadleaf,
    AnyTropicalBroadleaf,
    AnyWetland,
    AnyTemperateWetland,
    AnyTropicalWetland,
    AnyTropicalMarsh,
    AnyTemperateMarsh,
    AnyTropicalSwamp,
    AnyTemperateSwamp,
    AnyDesert,
    #[default]
    Unknown,
}
