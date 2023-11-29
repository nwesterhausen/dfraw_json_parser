use crate::RawMetadata;

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
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_one_u8(value: &u8) -> bool {
    *value == 1
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_one(value: &u32) -> bool {
    *value == 1
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero_u8(value: &u8) -> bool {
    *value == 0
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_sapling_drown_level(depth: &u8) -> bool {
    *depth == 4
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_tree_drown_level(depth: &u8) -> bool {
    *depth == 7
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_shrub_drown_level(depth: &u8) -> bool {
    *depth == 4
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_grow_duration(duration: &u32) -> bool {
    *duration == 300
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_cluster_size(size: &u32) -> bool {
    *size == 5
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_dead_shrub_tile(tile_code: &u8) -> bool {
    *tile_code == 34
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_shrub_tile(tile_code: &u8) -> bool {
    *tile_code == 34
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_dead_picked_tile(tile_code: &u8) -> bool {
    *tile_code == 169
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_default_picked_tile(tile_code: &u8) -> bool {
    *tile_code == 231
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_false(boolean: &bool) -> bool {
    !*boolean
}
/// This is only used for serialize
#[allow(dead_code)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_true(boolean: &bool) -> bool {
    *boolean
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_metadata_hidden(metadata: &Metadata) -> bool {
    metadata.is_hidden()
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero_i32(value: &i32) -> bool {
    *value == 0
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero_f32(value: &f32) -> bool {
    *value == 0.0
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_500_u32(value: &u32) -> bool {
    *value == 500
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_50_u32(value: &u32) -> bool {
    *value == 50
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_3_u32(value: &u32) -> bool {
    *value == 3
}
/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}
