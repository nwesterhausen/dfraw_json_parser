use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Static regex to re-use instead of re-make each time they're needed
    pub static ref RAW_TOKEN_RE: Regex = Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();
    pub static ref NON_DIGIT_RE: Regex = Regex::new(r"\D").unwrap();
    pub static ref NON_CHAR_RE: Regex = Regex::new(r"\W").unwrap();

    // The file encoding supported by dwarf fortress
    pub static ref DF_ENCODING: &'static encoding_rs::Encoding =
        encoding_rs::Encoding::for_label(b"latin1").unwrap();
}
