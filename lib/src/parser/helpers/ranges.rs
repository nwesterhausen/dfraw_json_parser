use std::num::ParseIntError;

use tracing::error;

/// It takes a slice of strings, parses the first two strings as unsigned 16-bit integers, and returns a
/// two-element array of unsigned 16-bit integers
///
/// Arguments:
///
/// * `split`: &[&str] - This is the array of strings that we're going to parse.
///
/// Returns:
///
/// A Result<[u16; 2], `ParseIntError`>
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range_from_vec(split: &[&str]) -> Result<[u16; 2], ParseIntError> {
    let min: u16 = match split.first().unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("min_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    let max: u16 = match split.get(1).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("max_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    Ok([min, max])
}

/// The function `parse_min_max_range` takes a string input and splits it by a colon, then calls another
/// function to parse the resulting vector into an array of two unsigned 16-bit integers.
///
/// Arguments:
///
/// * `value`: A string representing a range of values in the format "min:max".
///
/// Returns:
///
/// The function `parse_min_max_range` returns a `Result<[u16; 2], ParseIntError>`.
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range(value: &str) -> Result<[u16; 2], ParseIntError> {
    let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
    parse_min_max_range_from_vec(&split)
}
