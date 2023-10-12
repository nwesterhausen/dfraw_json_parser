use serde::{Deserialize, Serialize};

use crate::parser::{
    object_types::ObjectType,
    raws::{build_object_id_from_pieces, RawMetadata, RawObject},
    serializer_helper,
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectCreature {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    tags: Vec<String>,
}
impl SelectCreature {
    pub fn new(identifier: &str, metadata: &RawMetadata) -> SelectCreature {
        SelectCreature {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: build_object_id_from_pieces(
                metadata,
                identifier,
                &ObjectType::SelectCreature,
            ),
            ..SelectCreature::default()
        }
    }

    pub(crate) fn empty() -> Self {
        Self::default()
    }
}

#[typetag::serde]
impl RawObject for SelectCreature {
    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get_type(&self) -> &ObjectType {
        &ObjectType::SelectCreature
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.tags.push(format!("{key}:{value}"));
    }

    fn get_object_id(&self) -> &str {
        self.object_id.as_str()
    }
}
