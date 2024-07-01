//! Shared regexes for the library. These are used when parsing the raw object text files.

/// The regex used to match a raw token
pub static RAW_TOKEN_RE: lazy_regex::Lazy<lazy_regex::Regex> =
    lazy_regex::lazy_regex!(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])");
/// The regex for matching non-digit characters
pub static NON_DIGIT_RE: lazy_regex::Lazy<lazy_regex::Regex> = lazy_regex::lazy_regex!(r"\D");
/// The regex for matching non-character characters
pub static NON_CHAR_RE: lazy_regex::Lazy<lazy_regex::Regex> = lazy_regex::lazy_regex!(r"\W");
/// The regex for matching variation arguments
pub static VARIATION_ARGUMENT_RE: lazy_regex::Lazy<lazy_regex::Regex> =
    lazy_regex::lazy_regex!(r"(?m)!ARG(\d{1,3})");
