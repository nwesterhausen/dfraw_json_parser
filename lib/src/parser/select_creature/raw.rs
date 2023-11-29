use serde::{Deserialize, Serialize};

use crate::parser::{
    helpers::build_object_id_from_pieces,
<<<<<<< HEAD:lib/src/parser/select_creature/raw.rs
<<<<<<< HEAD:lib/src/parser/select_creature/raw.rs
    serializer_helper, ObjectType, {clean_search_vec, Searchable}, {RawMetadata, RawObject},
=======
=======
>>>>>>> c2812957821240fff30b78553e73f23e904207e2:src/parser/select_creature/raw.rs
    helpers::serializer_helper,
    metadata::Metadata,
    object_type::ObjectType,
    raws::RawObject,
    searchable::{clean_search_vec, Searchable},
<<<<<<< HEAD:lib/src/parser/select_creature/raw.rs
>>>>>>> 2b37a6f (refactor: expose 1 level down):src/parser/select_creature/raw.rs
=======
>>>>>>> c2812957821240fff30b78553e73f23e904207e2:src/parser/select_creature/raw.rs
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectCreature {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: Metadata,
    identifier: String,
    object_id: String,

    tags: Vec<String>,
}
impl SelectCreature {
    pub fn new(identifier: &str, metadata: &Metadata) -> SelectCreature {
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

    pub fn empty() -> Self {
        Self::default()
    }
}

#[typetag::serde]
impl RawObject for SelectCreature {
    fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
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

impl Searchable for SelectCreature {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.push("selectCreature".to_string());

        clean_search_vec(vec.as_slice())
    }
}
