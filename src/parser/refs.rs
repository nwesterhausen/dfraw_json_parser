use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// A static, pre-compiled regex for matching raw tokens, e.g. `[CREATURE:GOBLIN]`
    pub static ref RAW_TOKEN_RE: Regex = Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();
    /// A static, pre-compiled regex for matching non-digit characters, e.g. `a-z`, `A-Z`, `!`, etc.
    pub static ref NON_DIGIT_RE: Regex = Regex::new(r"\D").unwrap();
    /// A static, pre-compiled regex for matching non-character characters, e.g. `0-9`, `!`, etc.
    pub static ref NON_CHAR_RE: Regex = Regex::new(r"\W").unwrap();

    /// The file encoding supported by dwarf fortress, `latin1`.
    pub static ref DF_ENCODING: &'static encoding_rs::Encoding =
        encoding_rs::Encoding::for_label(b"latin1").unwrap();
}
