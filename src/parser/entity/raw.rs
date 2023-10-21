use serde::{Deserialize, Serialize};

use crate::parser::{
    object_types::ObjectType,
    raws::{RawMetadata, RawObject},
    searchable::{clean_search_vec, Searchable},
    serializer_helper,
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
        let Some(tag) = ENTITY_TOKENS.get(key) else {
            log::warn!(
                "Entity::parse_tag: called `Option::unwrap()` on a `None` value for presumed creature tag: {}",
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
