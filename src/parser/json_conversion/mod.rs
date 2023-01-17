pub trait TypedJsonSerializable {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error>;
}

mod creature;
mod info_file;
mod inorganic;
mod plant;

pub struct Empty;

impl TypedJsonSerializable for Empty {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(String::new())
    }
}
