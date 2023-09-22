use std::num::ParseIntError;

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
pub fn parse_min_max_range_from_vec(split: &Vec<&str>) -> Result<[u16; 2], ParseIntError> {
    let min: u16 = match split.first().unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            log::error!("min_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    let max: u16 = match split.get(1).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            log::error!("max_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    Ok([min, max])
}

pub fn parse_min_max_range(value: &str) -> Result<[u16; 2], ParseIntError> {
    let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
    parse_min_max_range_from_vec(&split)
}

/// It takes a slice of strings, parses them into integers, and returns a `DFBodySize` struct
///
/// Arguments:
///
/// * `split`: &[&str]
///
/// Returns:
///
/// A `Result<[u16; 3], ParseIntError>` of the 3 numbers in the body size
pub fn parse_body_size(split: &[&str]) -> Result<[u32; 3], ParseIntError> {
    let years: u32 = match split.first().unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            log::error!("Unable to parse years from BODY_SIZE\n{:?}", e);
            return Err(e);
        }
    };
    let days: u32 = match split.get(1).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            log::error!("Unable to parse days from BODY_SIZE\n{:?}", e);
            return Err(e);
        }
    };
    let size: u32 = match split.get(2).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            log::error!("Unable to parse size from BODY_SIZE\n{:?}", e);
            return Err(e);
        }
    };
    Ok([years, days, size])
}
