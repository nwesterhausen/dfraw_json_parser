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
pub fn parse_min_max_range_from_vec(split: &[&str]) -> Result<[u16; 2], ParseIntError> {
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

pub struct Ranges {}

impl Ranges {
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn min_max_is_ones(min_max: &[u16; 2]) -> bool {
        min_max[0] == 1 && min_max[1] == 1
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn min_max_is_zeroes(min_max: &[u16; 2]) -> bool {
        min_max[0] == 0 && min_max[1] == 0
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_zero(num: &u32) -> bool {
        *num == 0
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_zero_u8(num: &u8) -> bool {
        *num == 0
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_default_frequency(frequency: &u16) -> bool {
        *frequency == 50
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_default_trunk_height_percentage(values: &[i32; 2]) -> bool {
        values[0] == 0 && values[1] == -1
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_default_growth_density(density: &u32) -> bool {
        *density == 0
    }
    /// This is only used for serialize
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_default_growth_timing(values: &[u32; 2]) -> bool {
        values[0] == 0 && values[1] == 403_200
    }
}
