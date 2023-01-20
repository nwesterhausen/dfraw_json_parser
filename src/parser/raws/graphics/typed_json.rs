use crate::parser::TypedJsonSerializable;

use super::SpriteGraphic;

impl TypedJsonSerializable for SpriteGraphic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

impl TypedJsonSerializable for &SpriteGraphic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}
