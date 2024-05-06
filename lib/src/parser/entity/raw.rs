use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{
    clean_search_vec,
    helpers::build_object_id_from_pieces,
    position::{Position, TOKEN_MAP as POSITION_TOKEN_MAP},
    serializer_helper, Color, ObjectType, RawMetadata, RawObject, Searchable,
};

use super::{phf_table::ENTITY_TOKENS, tokens::EntityToken};

/// A struct representing an Entity object.
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    tags: Vec<EntityToken>,

    creature: Option<String>,
    translation: Option<String>,
    exclusive_start_biome: Option<String>,

    biome_support: Option<Vec<(String, u32)>>,
    settlement_biome: Option<Vec<String>>,
    start_biome: Option<Vec<String>>,
    likes_sites: Option<Vec<String>>,
    tolerates_sites: Option<Vec<String>>,
    world_constructions: Option<Vec<String>>,

    max_pop_number: Option<u32>,
    max_site_pop_number: Option<u32>,
    max_starting_civ_number: Option<u32>,

    permitted_buildings: Option<Vec<String>>,
    permitted_jobs: Option<Vec<String>>,
    permitted_reactions: Option<Vec<String>>,

    currency: Option<Vec<(String, u32)>>,
    art_facet_modifier: Option<Vec<(String, u32)>>,
    art_image_element_modifier: Option<Vec<(String, u32)>>,
    item_improvement_modifier: Option<Vec<(String, u32)>>,
    select_symbols: Option<Vec<(String, String)>>,
    subselect_symbols: Option<Vec<(String, String)>>,
    cull_symbols: Option<Vec<(String, String)>>,
    friendly_color: Option<Color>,

    religion: Option<String>,
    religion_spheres: Option<Vec<String>>,
    sphere_alignments: Option<Vec<String>>,

    positions: Option<Vec<Position>>,
    land_holder_trigger: Option<String>,
    site_variable_positions: Option<Vec<String>>,
    variable_positions: Option<Vec<String>>,

    ethics: Option<Vec<(String, String)>>,
    values: Option<Vec<(String, u32)>>,
    variable_values: Option<Vec<(String, u32, u32)>>,

    active_season: Option<String>,

    banditry: Option<f32>,

    progress_trigger_population: Option<u8>,
    progress_trigger_production: Option<u8>,
    progress_trigger_trade: Option<u8>,
    progress_trigger_population_siege: Option<u8>,
    progress_trigger_production_siege: Option<u8>,
    progress_trigger_trade_siege: Option<u8>,

    scholars: Option<Vec<String>>,

    ammo: Option<Vec<String>>,
    armors: Option<Vec<(String, u16)>>,
    diggers: Option<Vec<String>>,
    gloves: Option<Vec<(String, u16)>>,
    helms: Option<Vec<(String, u16)>>,
    instrument: Option<Vec<String>>,
    pants: Option<Vec<(String, u16)>>,
    shields: Option<Vec<String>>,
    shoes: Option<Vec<(String, u16)>>,
    siege_ammo: Option<Vec<String>>,
    tool: Option<Vec<String>>,
    toys: Option<Vec<String>>,
    trap_components: Option<Vec<String>>,
    weapons: Option<Vec<String>>,

    gem_shape: Option<Vec<String>>,
    stone_shape: Option<Vec<String>>,

    source_hfid: Option<u32>,
}

impl Entity {
    /// Function to create a new empty Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new empty Entity.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            // Default values which aren't rust defaults
            max_pop_number: Some(500),
            max_site_pop_number: Some(50),
            max_starting_civ_number: Some(3),

            ..Default::default()
        }
    }
    /// Function to create a new Entity.
    ///
    /// # Parameters
    ///
    /// * `identifier` - The identifier for the Entity.
    /// * `metadata` - The metadata for the Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new Entity.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Entity),
            // Default values which aren't rust defaults
            max_pop_number: Some(500),
            max_site_pop_number: Some(50),
            max_starting_civ_number: Some(3),
            ..Default::default()
        }
    }

    /// Function to "clean" the creature. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps:
    /// - Set the metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    ///
    /// # Returns
    ///
    /// * `Entity` - The cleaned Entity.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        // Remove metadata if hidden
        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        // Remove empty strings
        if cleaned.creature.as_deref() == Some("") {
            cleaned.creature = None;
        }
        if cleaned.translation.as_deref() == Some("") {
            cleaned.translation = None;
        }
        if cleaned.exclusive_start_biome.as_deref() == Some("") {
            cleaned.exclusive_start_biome = None;
        }

        if cleaned.biome_support.as_deref() == Some(&[]) {
            cleaned.biome_support = None;
        }
        if cleaned.settlement_biome.as_deref() == Some(&[]) {
            cleaned.settlement_biome = None;
        }
        if cleaned.start_biome.as_deref() == Some(&[]) {
            cleaned.start_biome = None;
        }
        if cleaned.likes_sites.as_deref() == Some(&[]) {
            cleaned.likes_sites = None;
        }
        if cleaned.tolerates_sites.as_deref() == Some(&[]) {
            cleaned.tolerates_sites = None;
        }
        if cleaned.world_constructions.as_deref() == Some(&[]) {
            cleaned.world_constructions = None;
        }

        if serializer_helper::is_500_u32(cleaned.max_pop_number) {
            cleaned.max_pop_number = None;
        }
        if serializer_helper::is_50_u32(cleaned.max_site_pop_number) {
            cleaned.max_site_pop_number = None;
        }
        if serializer_helper::is_3_u32(cleaned.max_starting_civ_number) {
            cleaned.max_starting_civ_number = None;
        }

        if cleaned.permitted_buildings.as_deref() == Some(&[]) {
            cleaned.permitted_buildings = None;
        }
        if cleaned.permitted_jobs.as_deref() == Some(&[]) {
            cleaned.permitted_jobs = None;
        }
        if cleaned.permitted_reactions.as_deref() == Some(&[]) {
            cleaned.permitted_reactions = None;
        }

        if cleaned.currency.as_deref() == Some(&[]) {
            cleaned.currency = None;
        }
        if cleaned.art_facet_modifier.as_deref() == Some(&[]) {
            cleaned.art_facet_modifier = None;
        }
        if cleaned.art_image_element_modifier.as_deref() == Some(&[]) {
            cleaned.art_image_element_modifier = None;
        }
        if cleaned.item_improvement_modifier.as_deref() == Some(&[]) {
            cleaned.item_improvement_modifier = None;
        }
        if cleaned.select_symbols.as_deref() == Some(&[]) {
            cleaned.select_symbols = None;
        }
        if cleaned.subselect_symbols.as_deref() == Some(&[]) {
            cleaned.subselect_symbols = None;
        }
        if cleaned.cull_symbols.as_deref() == Some(&[]) {
            cleaned.cull_symbols = None;
        }
        if let Some(color) = &cleaned.friendly_color {
            if color.is_default() {
                cleaned.friendly_color = None;
            }
        }

        if cleaned.religion.as_deref() == Some("") {
            cleaned.religion = None;
        }
        if cleaned.religion_spheres.as_deref() == Some(&[]) {
            cleaned.religion_spheres = None;
        }
        if cleaned.sphere_alignments.as_deref() == Some(&[]) {
            cleaned.sphere_alignments = None;
        }

        if let Some(positions) = &cleaned.positions {
            if positions.is_empty() {
                cleaned.positions = None;
            }
        }
        if cleaned.land_holder_trigger.as_deref() == Some("") {
            cleaned.land_holder_trigger = None;
        }
        if cleaned.site_variable_positions.as_deref() == Some(&[]) {
            cleaned.site_variable_positions = None;
        }
        if cleaned.variable_positions.as_deref() == Some(&[]) {
            cleaned.variable_positions = None;
        }

        if cleaned.ethics.as_deref() == Some(&[]) {
            cleaned.ethics = None;
        }
        if cleaned.values.as_deref() == Some(&[]) {
            cleaned.values = None;
        }
        if cleaned.variable_values.as_deref() == Some(&[]) {
            cleaned.variable_values = None;
        }

        if cleaned.active_season.as_deref() == Some("") {
            cleaned.active_season = None;
        }

        if serializer_helper::is_zero_f32(cleaned.banditry) {
            cleaned.banditry = None;
        }

        if serializer_helper::is_zero_u8(cleaned.progress_trigger_population) {
            cleaned.progress_trigger_population = None;
        }
        if serializer_helper::is_zero_u8(cleaned.progress_trigger_production) {
            cleaned.progress_trigger_production = None;
        }
        if serializer_helper::is_zero_u8(cleaned.progress_trigger_trade) {
            cleaned.progress_trigger_trade = None;
        }
        if serializer_helper::is_zero_u8(cleaned.progress_trigger_population_siege) {
            cleaned.progress_trigger_population_siege = None;
        }
        if serializer_helper::is_zero_u8(cleaned.progress_trigger_production_siege) {
            cleaned.progress_trigger_production_siege = None;
        }
        if serializer_helper::is_zero_u8(cleaned.progress_trigger_trade_siege) {
            cleaned.progress_trigger_trade_siege = None;
        }

        if cleaned.scholars.as_deref() == Some(&[]) {
            cleaned.scholars = None;
        }
        if cleaned.ammo.as_deref() == Some(&[]) {
            cleaned.ammo = None;
        }
        if cleaned.armors.as_deref() == Some(&[]) {
            cleaned.armors = None;
        }
        if cleaned.diggers.as_deref() == Some(&[]) {
            cleaned.diggers = None;
        }
        if cleaned.gloves.as_deref() == Some(&[]) {
            cleaned.gloves = None;
        }
        if cleaned.helms.as_deref() == Some(&[]) {
            cleaned.helms = None;
        }
        if cleaned.instrument.as_deref() == Some(&[]) {
            cleaned.instrument = None;
        }
        if cleaned.pants.as_deref() == Some(&[]) {
            cleaned.pants = None;
        }
        if cleaned.shields.as_deref() == Some(&[]) {
            cleaned.shields = None;
        }
        if cleaned.shoes.as_deref() == Some(&[]) {
            cleaned.shoes = None;
        }
        if cleaned.siege_ammo.as_deref() == Some(&[]) {
            cleaned.siege_ammo = None;
        }
        if cleaned.tool.as_deref() == Some(&[]) {
            cleaned.tool = None;
        }
        if cleaned.toys.as_deref() == Some(&[]) {
            cleaned.toys = None;
        }
        if cleaned.trap_components.as_deref() == Some(&[]) {
            cleaned.trap_components = None;
        }
        if cleaned.weapons.as_deref() == Some(&[]) {
            cleaned.weapons = None;
        }

        if cleaned.gem_shape.as_deref() == Some(&[]) {
            cleaned.gem_shape = None;
        }
        if cleaned.stone_shape.as_deref() == Some(&[]) {
            cleaned.stone_shape = None;
        }

        if serializer_helper::is_zero_u32(cleaned.source_hfid) {
            cleaned.source_hfid = None;
        }

        cleaned
    }
}

#[typetag::serde]
impl RawObject for Entity {
    fn get_object_id(&self) -> &str {
        self.object_id.as_str()
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!("Entity::get_metadata: no metadata for {}", self.identifier);
                RawMetadata::default()
                    .with_object_type(ObjectType::Entity)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
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
    fn clean_self(&mut self) {
        *self = self.cleaned();
    }
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if let Some(position_token) = POSITION_TOKEN_MAP.get(key) {
            if self.positions.is_none() {
                self.positions = Some(Vec::new());
            }

            if let Some(positions) = self.positions.as_mut() {
                // Tags should be attached to the last Position in the list
                if let Some(position) = positions.last_mut() {
                    position.parse_tag(position_token, value);
                    return;
                }
                // If there is no position, create one with unknown name..
                let mut position = Position::new("unknown".into());
                position.parse_tag(position_token, value);
                positions.push(position);
                return;
            }
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
                self.active_season = Some(value.to_string());
            }
            EntityToken::Banditry => {
                self.banditry = Some(value.parse().unwrap_or_default());
            }
            EntityToken::Creature => {
                self.creature = Some(value.to_string());
            }
            EntityToken::ProgressTriggerPopulation => {
                self.progress_trigger_population = Some(value.parse().unwrap_or_default());
            }
            EntityToken::ProgressTriggerProduction => {
                self.progress_trigger_production = Some(value.parse().unwrap_or_default());
            }
            EntityToken::ProgressTriggerTrade => {
                self.progress_trigger_trade = Some(value.parse().unwrap_or_default());
            }
            EntityToken::ProgressTriggerPopulationSiege => {
                self.progress_trigger_population_siege = Some(value.parse().unwrap_or_default());
            }
            EntityToken::ProgressTriggerProductionSiege => {
                self.progress_trigger_production_siege = Some(value.parse().unwrap_or_default());
            }
            EntityToken::ProgressTriggerTradeSiege => {
                self.progress_trigger_trade_siege = Some(value.parse().unwrap_or_default());
            }
            EntityToken::Scholar => {
                if let Some(scholars) = &mut self.scholars {
                    scholars.push(value.to_string());
                } else {
                    self.scholars = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Ammo => {
                if let Some(ammo) = &mut self.ammo {
                    ammo.push(value.to_string());
                } else {
                    self.ammo = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Armor => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(armors) = &mut self.armors {
                    armors.push((armor, chance));
                } else {
                    self.armors = Some(vec![(armor, chance)]);
                }
            }
            EntityToken::Digger => {
                if let Some(diggers) = &mut self.diggers {
                    diggers.push(value.to_string());
                } else {
                    self.diggers = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Gloves => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(gloves) = &mut self.gloves {
                    gloves.push((armor, chance));
                } else {
                    self.gloves = Some(vec![(armor, chance)]);
                }
            }
            EntityToken::Helm => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(helms) = &mut self.helms {
                    helms.push((armor, chance));
                } else {
                    self.helms = Some(vec![(armor, chance)]);
                }
            }
            EntityToken::Instrument => {
                if let Some(instrument) = &mut self.instrument {
                    instrument.push(value.to_string());
                } else {
                    self.instrument = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Pants => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(pants) = &mut self.pants {
                    pants.push((armor, chance));
                } else {
                    self.pants = Some(vec![(armor, chance)]);
                }
            }
            EntityToken::Shield => {
                if let Some(shields) = &mut self.shields {
                    shields.push(value.to_string());
                } else {
                    self.shields = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Shoes => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(shoes) = &mut self.shoes {
                    shoes.push((armor, chance));
                } else {
                    self.shoes = Some(vec![(armor, chance)]);
                }
            }
            EntityToken::SiegeAmmo => {
                if let Some(siege_ammo) = &mut self.siege_ammo {
                    siege_ammo.push(value.to_string());
                } else {
                    self.siege_ammo = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Tool => {
                if let Some(tool) = &mut self.tool {
                    tool.push(value.to_string());
                } else {
                    self.tool = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Toy => {
                if let Some(toys) = &mut self.toys {
                    toys.push(value.to_string());
                } else {
                    self.toys = Some(vec![value.to_string()]);
                }
            }
            EntityToken::TrapComponent => {
                if let Some(trap_components) = &mut self.trap_components {
                    trap_components.push(value.to_string());
                } else {
                    self.trap_components = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Weapon => {
                if let Some(weapons) = &mut self.weapons {
                    weapons.push(value.to_string());
                } else {
                    self.weapons = Some(vec![value.to_string()]);
                }
            }
            EntityToken::GemShape => {
                if let Some(gem_shape) = &mut self.gem_shape {
                    gem_shape.push(value.to_string());
                } else {
                    self.gem_shape = Some(vec![value.to_string()]);
                }
            }
            EntityToken::StoneShape => {
                if let Some(stone_shape) = &mut self.stone_shape {
                    stone_shape.push(value.to_string());
                } else {
                    self.stone_shape = Some(vec![value.to_string()]);
                }
            }
            EntityToken::BiomeSupport => {
                let mut split = value.split(':');
                let biome = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(biome_support) = &mut self.biome_support {
                    biome_support.push((biome, chance));
                } else {
                    self.biome_support = Some(vec![(biome, chance)]);
                }
            }
            EntityToken::SettlementBiome => {
                if let Some(settlement_biome) = &mut self.settlement_biome {
                    settlement_biome.push(value.to_string());
                } else {
                    self.settlement_biome = Some(vec![value.to_string()]);
                }
            }
            EntityToken::StartBiome => {
                if let Some(start_biome) = &mut self.start_biome {
                    start_biome.push(value.to_string());
                } else {
                    self.start_biome = Some(vec![value.to_string()]);
                }
            }
            EntityToken::LikesSite => {
                if let Some(likes_sites) = &mut self.likes_sites {
                    likes_sites.push(value.to_string());
                } else {
                    self.likes_sites = Some(vec![value.to_string()]);
                }
            }
            EntityToken::ToleratesSite => {
                if let Some(tolerates_sites) = &mut self.tolerates_sites {
                    tolerates_sites.push(value.to_string());
                } else {
                    self.tolerates_sites = Some(vec![value.to_string()]);
                }
            }
            EntityToken::WorldConstruction => {
                if let Some(world_constructions) = &mut self.world_constructions {
                    world_constructions.push(value.to_string());
                } else {
                    self.world_constructions = Some(vec![value.to_string()]);
                }
            }
            EntityToken::PermittedBuilding => {
                if let Some(permitted_buildings) = &mut self.permitted_buildings {
                    permitted_buildings.push(value.to_string());
                } else {
                    self.permitted_buildings = Some(vec![value.to_string()]);
                }
            }
            EntityToken::PermittedJob => {
                if let Some(permitted_jobs) = &mut self.permitted_jobs {
                    permitted_jobs.push(value.to_string());
                } else {
                    self.permitted_jobs = Some(vec![value.to_string()]);
                }
            }
            EntityToken::PermittedReaction => {
                if let Some(permitted_reactions) = &mut self.permitted_reactions {
                    permitted_reactions.push(value.to_string());
                } else {
                    self.permitted_reactions = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Currency => {
                let mut split = value.split(':');
                let currency = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(self_currency) = &mut self.currency {
                    self_currency.push((currency, chance));
                } else {
                    self.currency = Some(vec![(currency, chance)]);
                }
            }
            EntityToken::ArtFacetModifier => {
                let mut split = value.split(':');
                let facet = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(art_facet_modifier) = &mut self.art_facet_modifier {
                    art_facet_modifier.push((facet, chance));
                } else {
                    self.art_facet_modifier = Some(vec![(facet, chance)]);
                }
            }
            EntityToken::ArtImageElementModifier => {
                let mut split = value.split(':');
                let element = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(art_image_element_modifier) = &mut self.art_image_element_modifier {
                    art_image_element_modifier.push((element, chance));
                } else {
                    self.art_image_element_modifier = Some(vec![(element, chance)]);
                }
            }
            EntityToken::ItemImprovementModifier => {
                let mut split = value.split(':');
                let improvement = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(item_improvement_modifier) = &mut self.item_improvement_modifier {
                    item_improvement_modifier.push((improvement, chance));
                } else {
                    self.item_improvement_modifier = Some(vec![(improvement, chance)]);
                }
            }
            EntityToken::SelectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(select_symbols) = &mut self.select_symbols {
                    select_symbols.push((symbol, chance));
                } else {
                    self.select_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityToken::SubselectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(subselect_symbols) = &mut self.subselect_symbols {
                    subselect_symbols.push((symbol, chance));
                } else {
                    self.subselect_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityToken::CullSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(cull_symbols) = &mut self.cull_symbols {
                    cull_symbols.push((symbol, chance));
                } else {
                    self.cull_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityToken::FriendlyColor => {
                self.friendly_color = Some(Color::from_value(value));
            }
            EntityToken::Religion => {
                self.religion = Some(value.to_string());
            }
            EntityToken::ReligionSphere => {
                if let Some(religion_spheres) = &mut self.religion_spheres {
                    religion_spheres.push(value.to_string());
                } else {
                    self.religion_spheres = Some(vec![value.to_string()]);
                }
            }
            EntityToken::SphereAlignment => {
                if let Some(sphere_alignments) = &mut self.sphere_alignments {
                    sphere_alignments.push(value.to_string());
                } else {
                    self.sphere_alignments = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Position => {
                let position = Position::new(value.to_string());
                if let Some(positions) = &mut self.positions {
                    positions.push(position);
                } else {
                    self.positions = Some(vec![position]);
                }
            }
            EntityToken::LandHolderTrigger => {
                self.land_holder_trigger = Some(value.to_string());
            }
            EntityToken::SiteVariablePositions => {
                if let Some(site_variable_positions) = &mut self.site_variable_positions {
                    site_variable_positions.push(value.to_string());
                } else {
                    self.site_variable_positions = Some(vec![value.to_string()]);
                }
            }
            EntityToken::VariablePositions => {
                if let Some(variable_positions) = &mut self.variable_positions {
                    variable_positions.push(value.to_string());
                } else {
                    self.variable_positions = Some(vec![value.to_string()]);
                }
            }
            EntityToken::Ethic => {
                let mut split = value.split(':');
                let ethic = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(ethics) = &mut self.ethics {
                    ethics.push((ethic, chance));
                } else {
                    self.ethics = Some(vec![(ethic, chance)]);
                }
            }
            EntityToken::Value => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(values) = &mut self.values {
                    values.push((value, chance));
                } else {
                    self.values = Some(vec![(value, chance)]);
                }
            }
            EntityToken::VariableValue => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                let max = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(variable_values) = &mut self.variable_values {
                    variable_values.push((value, chance, max));
                } else {
                    self.variable_values = Some(vec![(value, chance, max)]);
                }
            }
            EntityToken::ExclusiveStartBiome => {
                self.exclusive_start_biome = Some(value.to_string());
            }
            EntityToken::MaxPopNumber => {
                self.max_pop_number = Some(value.parse().unwrap_or_default());
            }
            EntityToken::MaxSitePopNumber => {
                self.max_site_pop_number = Some(value.parse().unwrap_or_default());
            }
            EntityToken::MaxStartingCivNumber => {
                self.max_starting_civ_number = Some(value.parse().unwrap_or_default());
            }
            EntityToken::SourceHfid => {
                self.source_hfid = Some(value.parse().unwrap_or_default());
            }
            EntityToken::Translation => {
                self.translation = Some(value.to_string());
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
        if let Some(scholars) = &self.scholars {
            vec.extend(scholars.iter().cloned());
        }
        // vec.extend(self.ammo.iter().cloned());
        if let Some(ammo) = &self.ammo {
            vec.extend(ammo.iter().cloned());
        }
        // vec.extend(self.armors.iter().map(|(armor, _)| format!("{armor:?}")));
        if let Some(armors) = &self.armors {
            vec.extend(armors.iter().map(|(armor, _)| format!("{armor:?}")));
        }
        // vec.extend(self.diggers.iter().cloned());
        if let Some(diggers) = &self.diggers {
            vec.extend(diggers.iter().cloned());
        }
        // vec.extend(self.gloves.iter().map(|(glove, _)| format!("{glove:?}")));
        if let Some(gloves) = &self.gloves {
            vec.extend(gloves.iter().map(|(glove, _)| format!("{glove:?}")));
        }
        // vec.extend(self.helms.iter().map(|(helm, _)| format!("{helm:?}")));
        if let Some(helms) = &self.helms {
            vec.extend(helms.iter().map(|(helm, _)| format!("{helm:?}")));
        }
        // vec.extend(self.instrument.iter().cloned());
        if let Some(instrument) = &self.instrument {
            vec.extend(instrument.iter().cloned());
        }
        // vec.extend(self.pants.iter().map(|(pants, _)| format!("{pants:?}")));
        if let Some(pants) = &self.pants {
            vec.extend(pants.iter().map(|(pants, _)| format!("{pants:?}")));
        }
        // vec.extend(self.shields.iter().cloned());
        if let Some(shields) = &self.shields {
            vec.extend(shields.iter().cloned());
        }
        // vec.extend(self.shoes.iter().map(|(shoe, _)| format!("{shoe:?}")));
        if let Some(shoes) = &self.shoes {
            vec.extend(shoes.iter().map(|(shoe, _)| format!("{shoe:?}")));
        }
        // vec.extend(self.siege_ammo.iter().cloned());
        if let Some(siege_ammo) = &self.siege_ammo {
            vec.extend(siege_ammo.iter().cloned());
        }
        // vec.extend(self.tool.iter().cloned());
        if let Some(tool) = &self.tool {
            vec.extend(tool.iter().cloned());
        }
        // vec.extend(self.toys.iter().cloned());
        if let Some(toys) = &self.toys {
            vec.extend(toys.iter().cloned());
        }
        // vec.extend(self.trap_components.iter().cloned());
        if let Some(trap_components) = &self.trap_components {
            vec.extend(trap_components.iter().cloned());
        }
        // vec.extend(self.weapons.iter().cloned());
        if let Some(weapons) = &self.weapons {
            vec.extend(weapons.iter().cloned());
        }
        // vec.extend(self.gem_shape.iter().cloned());
        if let Some(gem_shape) = &self.gem_shape {
            vec.extend(gem_shape.iter().cloned());
        }
        // vec.extend(self.stone_shape.iter().cloned());
        if let Some(stone_shape) = &self.stone_shape {
            vec.extend(stone_shape.iter().cloned());
        }

        clean_search_vec(vec.as_slice())
    }
}
