use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::parser::{
    clean_search_vec, material::Material, ObjectType, RawMetadata, RawObject, Searchable,
};



#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTemplate {
    identifier: String,
    metadata: Option<RawMetadata>,
    object_id: String,
    material: Material,
}

impl MaterialTemplate {
    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::MaterialTemplate)
                    .with_hidden(true),
            ),
            ..Self::default()
        }
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> MaterialTemplate {
        MaterialTemplate {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: format!(
                "{}-{}-{}",
                metadata.get_raw_identifier(),
                "MATERIAL_TEMPLATE",
                slugify(identifier)
            ),
            ..MaterialTemplate::default()
        }
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        cleaned
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

    fn get_metadata(&self) -> RawMetadata {
        if let Some(metadata) = &self.metadata {
            metadata.clone()
        } else {
            tracing::warn!(
                "Metadata is missing for MaterialTemplate {}",
                self.get_object_id()
            );
            RawMetadata::default()
                .with_object_type(ObjectType::MaterialTemplate)
                .with_hidden(true)
        }
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
    fn clean_self(&mut self) {
        *self = self.cleaned();
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
