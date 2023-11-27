use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    material::Material,
    object_types::ObjectType,
    raws::{RawMetadata, RawObject},
    searchable::{clean_search_vec, Searchable},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTemplate {
    identifier: String,
    metadata: RawMetadata,
    object_id: String,
    material: Material,
}

impl MaterialTemplate {
    pub fn empty() -> MaterialTemplate {
        MaterialTemplate::default()
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> MaterialTemplate {
        MaterialTemplate {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "MATERIAL_TEMPLATE",
                slugify(identifier)
            ),
            ..MaterialTemplate::default()
        }
    }
}

#[typetag::serde]
impl RawObject for MaterialTemplate {
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }

    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.material.parse_tag(key, value);
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::MaterialTemplate
    }
}

impl Searchable for MaterialTemplate {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.extend(self.material.get_search_vec());
        vec.push("template".to_string());

        clean_search_vec(vec.as_slice())
    }
}
