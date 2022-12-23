pub trait TypedJsonSerializable {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error>;
}

mod creature;
mod info_file;

pub struct Empty;

impl TypedJsonSerializable for Empty {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        Ok("".to_owned())
    }
}
