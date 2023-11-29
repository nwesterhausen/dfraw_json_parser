use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{
    clean_search_vec,
    helpers::build_object_id_from_pieces,
    position::{Position, TOKEN_MAP as POSITION_TOKEN_MAP},
    serializer_helper, Color, ObjectType, RawMetadata, RawObject, Searchable,
};

use super::{phf_table::ENTITY_TOKENS, tokens::EntityToken};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    tags: Vec<EntityToken>,

    #[serde(skip_serializing_if = "String::is_empty")]
    creature: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    translation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    exclusive_start_biome: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    biome_support: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    settlement_biome: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    start_biome: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    likes_sites: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tolerates_sites: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    world_constructions: Vec<String>,

    #[serde(skip_serializing_if = "serializer_helper::is_500_u32")]
    max_pop_number: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_50_u32")]
    max_site_pop_number: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_3_u32")]
    max_starting_civ_number: u32,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    permitted_buildings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    permitted_jobs: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    permitted_reactions: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    currency: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    art_facet_modifier: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    art_image_element_modifier: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    item_improvement_modifier: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    select_symbols: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subselect_symbols: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cull_symbols: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Color::is_default")]
    friendly_color: Color,

    #[serde(skip_serializing_if = "String::is_empty")]
    religion: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    religion_spheres: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    sphere_alignments: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    positions: Vec<Position>,
    #[serde(skip_serializing_if = "String::is_empty")]
    land_holder_trigger: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    site_variable_positions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    variable_positions: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    ethics: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    values: Vec<(String, u32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    variable_values: Vec<(String, u32, u32)>,

    #[serde(skip_serializing_if = "String::is_empty")]
    active_season: String,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_f32")]
    banditry: f32,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_population: u8,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_production: u8,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_trade: u8,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_population_siege: u8,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_production_siege: u8,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    progress_trigger_trade_siege: u8,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    scholars: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    ammo: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    armors: Vec<(String, u16)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    diggers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    gloves: Vec<(String, u16)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    helms: Vec<(String, u16)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    instrument: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pants: Vec<(String, u16)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    shields: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    shoes: Vec<(String, u16)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    siege_ammo: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tool: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    toys: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    trap_components: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    weapons: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    gem_shape: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stone_shape: Vec<String>,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    source_hfid: u32,
}

impl Entity {
    pub fn empty() -> Self {
        Entity {
            // Default values which aren't rust defaults
            max_pop_number: 500,
            max_site_pop_number: 50,
            max_starting_civ_number: 3,

            ..Default::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Entity {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Entity),
            // Default values which aren't rust defaults
            max_pop_number: 500,
            max_site_pop_number: 50,
            max_starting_civ_number: 3,
            ..Default::default()
        }
    }
}

#[typetag::serde]
impl RawObject for Entity {
    fn get_object_id(&self) -> &str {
        self.object_id.as_str()
    }
    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::Entity
    }
    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if let Some(position_token) = POSITION_TOKEN_MAP.get(key) {
            // Tags should be attached to the last Position in the list
            if let Some(position) = self.positions.last_mut() {
                position.parse_tag(position_token, value);
                return;
            }
            // If there is no position, create one with unknown name..
            let mut position = Position::new("unknown".into());
            position.parse_tag(position_token, value);
            self.positions.push(position);
            return;
        }

        let Some(tag) = ENTITY_TOKENS.get(key) else {
            warn!(
                "Entity::parse_tag: called `Option::unwrap()` on a `None` value for presumed Entity tag: {}",
                key
            );
            return;
        };

        match tag {
            EntityToken::ActiveSeason => {
                self.active_season = value.to_string();
            }
            EntityToken::Banditry => {
                self.banditry = value.parse().unwrap_or_default();
            }
            EntityToken::Creature => {
                self.creature = value.to_string();
            }
            EntityToken::ProgressTriggerPopulation => {
                self.progress_trigger_population = value.parse().unwrap_or_default();
            }
            EntityToken::ProgressTriggerProduction => {
                self.progress_trigger_production = value.parse().unwrap_or_default();
            }
            EntityToken::ProgressTriggerTrade => {
                self.progress_trigger_trade = value.parse().unwrap_or_default();
            }
            EntityToken::ProgressTriggerPopulationSiege => {
                self.progress_trigger_population_siege = value.parse().unwrap_or_default();
            }
            EntityToken::ProgressTriggerProductionSiege => {
                self.progress_trigger_production_siege = value.parse().unwrap_or_default();
            }
            EntityToken::ProgressTriggerTradeSiege => {
                self.progress_trigger_trade_siege = value.parse().unwrap_or_default();
            }
            EntityToken::Scholar => {
                self.scholars.push(value.to_string());
            }
            EntityToken::Ammo => {
                self.ammo.push(value.to_string());
            }
            EntityToken::Armor => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.armors.push((armor, chance));
            }
            EntityToken::Digger => {
                self.diggers.push(value.to_string());
            }
            EntityToken::Gloves => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.gloves.push((armor, chance));
            }
            EntityToken::Helm => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.helms.push((armor, chance));
            }
            EntityToken::Instrument => {
                self.instrument.push(value.to_string());
            }
            EntityToken::Pants => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.pants.push((armor, chance));
            }
            EntityToken::Shield => {
                self.shields.push(value.to_string());
            }
            EntityToken::Shoes => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.shoes.push((armor, chance));
            }
            EntityToken::SiegeAmmo => {
                self.siege_ammo.push(value.to_string());
            }
            EntityToken::Tool => {
                self.tool.push(value.to_string());
            }
            EntityToken::Toy => {
                self.toys.push(value.to_string());
            }
            EntityToken::TrapComponent => {
                self.trap_components.push(value.to_string());
            }
            EntityToken::Weapon => {
                self.weapons.push(value.to_string());
            }
            EntityToken::GemShape => {
                self.gem_shape.push(value.to_string());
            }
            EntityToken::StoneShape => {
                self.stone_shape.push(value.to_string());
            }
            EntityToken::BiomeSupport => {
                let mut split = value.split(':');
                let biome = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.biome_support.push((biome, chance));
            }
            EntityToken::SettlementBiome => {
                self.settlement_biome.push(value.to_string());
            }
            EntityToken::StartBiome => {
                self.start_biome.push(value.to_string());
            }
            EntityToken::LikesSite => {
                self.likes_sites.push(value.to_string());
            }
            EntityToken::ToleratesSite => {
                self.tolerates_sites.push(value.to_string());
            }
            EntityToken::WorldConstruction => {
                self.world_constructions.push(value.to_string());
            }
            EntityToken::PermittedBuilding => {
                self.permitted_buildings.push(value.to_string());
            }
            EntityToken::PermittedJob => {
                self.permitted_jobs.push(value.to_string());
            }
            EntityToken::PermittedReaction => {
                self.permitted_reactions.push(value.to_string());
            }
            EntityToken::Currency => {
                let mut split = value.split(':');
                let currency = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.currency.push((currency, chance));
            }
            EntityToken::ArtFacetModifier => {
                let mut split = value.split(':');
                let facet = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.art_facet_modifier.push((facet, chance));
            }
            EntityToken::ArtImageElementModifier => {
                let mut split = value.split(':');
                let element = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.art_image_element_modifier.push((element, chance));
            }
            EntityToken::ItemImprovementModifier => {
                let mut split = value.split(':');
                let improvement = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.item_improvement_modifier.push((improvement, chance));
            }
            EntityToken::SelectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();
                self.select_symbols.push((symbol, chance));
            }
            EntityToken::SubselectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();
                self.subselect_symbols.push((symbol, chance));
            }
            EntityToken::CullSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();
                self.cull_symbols.push((symbol, chance));
            }
            EntityToken::FriendlyColor => {
                self.friendly_color = Color::from_value(value);
            }
            EntityToken::Religion => {
                self.religion = value.to_string();
            }
            EntityToken::ReligionSphere => {
                self.religion_spheres.push(value.to_string());
            }
            EntityToken::SphereAlignment => {
                self.sphere_alignments.push(value.to_string());
            }
            EntityToken::Position => {
                self.positions.push(Position::new(value.into()));
            }
            EntityToken::LandHolderTrigger => {
                self.land_holder_trigger = value.to_string();
            }
            EntityToken::SiteVariablePositions => {
                self.site_variable_positions.push(value.to_string());
            }
            EntityToken::VariablePositions => {
                self.variable_positions.push(value.to_string());
            }
            EntityToken::Ethic => {
                let mut split = value.split(':');
                let ethic = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();
                self.ethics.push((ethic, chance));
            }
            EntityToken::Value => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.values.push((value, chance));
            }
            EntityToken::VariableValue => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                let max = split.next().unwrap_or_default().parse().unwrap_or_default();
                self.variable_values.push((value, chance, max));
            }
            EntityToken::ExclusiveStartBiome => {
                self.exclusive_start_biome = value.to_string();
            }
            EntityToken::MaxPopNumber => {
                self.max_pop_number = value.parse().unwrap_or_default();
            }
            EntityToken::MaxSitePopNumber => {
                self.max_site_pop_number = value.parse().unwrap_or_default();
            }
            EntityToken::MaxStartingCivNumber => {
                self.max_starting_civ_number = value.parse().unwrap_or_default();
            }
            EntityToken::SourceHfid => {
                self.source_hfid = value.parse().unwrap_or_default();
            }
            EntityToken::Translation => {
                self.translation = value.to_string();
            }

            _ => {
                self.tags.push(tag.clone());
            }
        }
    }
}

impl Searchable for Entity {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.identifier.clone());
        vec.extend(self.tags.iter().map(|tag| format!("{tag:?}")));
        vec.extend(self.scholars.iter().cloned());
        vec.extend(self.ammo.iter().cloned());
        vec.extend(self.armors.iter().map(|(armor, _)| format!("{armor:?}")));
        vec.extend(self.diggers.iter().cloned());
        vec.extend(self.gloves.iter().map(|(glove, _)| format!("{glove:?}")));
        vec.extend(self.helms.iter().map(|(helm, _)| format!("{helm:?}")));
        vec.extend(self.instrument.iter().cloned());
        vec.extend(self.pants.iter().map(|(pants, _)| format!("{pants:?}")));
        vec.extend(self.shields.iter().cloned());
        vec.extend(self.shoes.iter().map(|(shoe, _)| format!("{shoe:?}")));
        vec.extend(self.siege_ammo.iter().cloned());
        vec.extend(self.tool.iter().cloned());
        vec.extend(self.toys.iter().cloned());
        vec.extend(self.trap_components.iter().cloned());
        vec.extend(self.weapons.iter().cloned());
        vec.extend(self.gem_shape.iter().cloned());
        vec.extend(self.stone_shape.iter().cloned());

        clean_search_vec(vec.as_slice())
    }
}
