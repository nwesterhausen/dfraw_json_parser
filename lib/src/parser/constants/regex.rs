use regex::Regex;

lazy_static::lazy_static! {
    /// The regex used to match a raw token
    pub static ref RAW_TOKEN_RE: Regex =
        Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap_or_else(|e| {
            panic!("Failed to compile RAW_TOKEN_RE: {e}");
        });
    /// The regex for matching non-digit characters
    pub static ref NON_DIGIT_RE: Regex = Regex::new(r"\D").unwrap_or_else(|e| {
        panic!("Failed to compile NON_DIGIT_RE: {e}");
    });
    /// The regex for matching non-character characters
    pub static ref NON_CHAR_RE: Regex = Regex::new(r"\W").unwrap_or_else(|e| {
        panic!("Failed to compile NON_CHAR_RE: {e}");
    });
    /// The regex for matching variation arguments
    pub static ref VARIATION_ARGUMENT_RE: Regex = Regex::new(r"(?m)!ARG(\d{1,3})").unwrap_or_else(|e| {
        panic!("Failed to compile VARIATION_ARGUMENT_RE: {e}");
    });
}
