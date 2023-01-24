pub mod inits;
mod parsing_bits;
mod parsing_info_txt;
mod parsing_to_json;
mod parsing_to_serializable;
pub mod raws;
mod reader;
mod refs;

#[allow(clippy::module_name_repetitions)]
pub struct DFParser;

pub trait TypedJsonSerializable {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error>;
}

pub trait RawsStyleSerializable {
    fn to_raws_style(&self) -> String;
}
