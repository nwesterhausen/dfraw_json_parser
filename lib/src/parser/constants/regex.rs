use regex::Regex;

lazy_static::lazy_static! {
    pub static ref RAW_TOKEN_RE: Regex =
        Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();
    pub static ref NON_DIGIT_RE: Regex = Regex::new(r"\D").unwrap();
    pub static ref NON_CHAR_RE: Regex = Regex::new(r"\W").unwrap();
}
