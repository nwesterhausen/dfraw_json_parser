use crate::RawMetadata;

/// ``min_max[0]`` == 1 && `min_max[1]` == 1
pub const fn min_max_is_ones(min_max: &Option<[u32; 2]>) -> bool {
    if let Some(min_max) = min_max {
        return min_max[0] == 1 && min_max[1] == 1;
    }
    true
}
/// `min_max[0]` == 0 && `min_max[1]` == 0
pub const fn min_max_is_zeroes(min_max: &Option<[u32; 2]>) -> bool {
    if let Some(min_max) = min_max {
        return min_max[0] == 0 && min_max[1] == 0;
    }
    true
}
/// This is only used for serialize
pub const fn is_zero(num: Option<u32>) -> bool {
    if let Some(num) = num {
        return num == 0;
    }
    true
}
/// This is only used for serialize
pub const fn is_default_frequency(frequency: Option<u32>) -> bool {
    if let Some(frequency) = frequency {
        return frequency == 50;
    }
    true
}
/// values[0] == 0 && values[1] == -1
pub const fn is_default_trunk_height_percentage(values: &Option<[i32; 2]>) -> bool {
    if let Some(values) = values {
        return values[0] == 0 && values[1] == -1;
    }
    true
}
/// This is only used for serialize
pub const fn is_default_growth_density(density: Option<u32>) -> bool {
    if let Some(density) = density {
        return density == 0;
    }
    true
}
/// `values[0]` == `0` && `values[1]` == `403_200`
pub const fn is_default_growth_timing(values: &Option<[u32; 2]>) -> bool {
    if let Some(values) = values {
        return values[0] == 0 && values[1] == 403_200;
    }
    true
}
/// This is only used for serialize
pub const fn is_one_u8(value: Option<u8>) -> bool {
    if let Some(value) = value {
        return value == 1;
    }
    true
}
/// This is only used for serialize
pub const fn is_one(value: Option<u32>) -> bool {
    if let Some(value) = value {
        return value == 1;
    }
    true
}
/// This is only used for serialize
pub const fn is_zero_u8(value: Option<u8>) -> bool {
    if let Some(value) = value {
        return value == 0;
    }
    true
}
/// depth == 4
pub const fn is_default_sapling_drown_level(depth: Option<u8>) -> bool {
    if let Some(depth) = depth {
        return depth == 4;
    }
    true
}
/// depth == 7
pub const fn is_default_tree_drown_level(depth: Option<u8>) -> bool {
    if let Some(depth) = depth {
        return depth == 7;
    }
    true
}
/// depth == 4
pub const fn is_default_shrub_drown_level(depth: Option<u8>) -> bool {
    if let Some(depth) = depth {
        return depth == 4;
    }
    true
}

/// duration == 300
pub const fn is_default_grow_duration(duration: Option<u32>) -> bool {
    if let Some(duration) = duration {
        return duration == 300;
    }
    true
}
/// size == 5
pub const fn is_default_cluster_size(size: Option<u32>) -> bool {
    if let Some(size) = size {
        return size == 5;
    }
    true
}

/// `tile_code` == 34
pub const fn is_default_dead_shrub_tile(tile_code: Option<u8>) -> bool {
    if let Some(tile_code) = tile_code {
        return tile_code == 34;
    }
    true
}

/// `tile_code` == 34
pub const fn is_default_shrub_tile(tile_code: Option<u8>) -> bool {
    if let Some(tile_code) = tile_code {
        return tile_code == 34;
    }
    true
}

/// `tile_code` == 169
pub const fn is_default_dead_picked_tile(tile_code: Option<u8>) -> bool {
    if let Some(tile_code) = tile_code {
        return tile_code == 169;
    }
    true
}
/// `tile_code` == 231
pub const fn is_default_picked_tile(tile_code: Option<u8>) -> bool {
    if let Some(tile_code) = tile_code {
        return tile_code == 231;
    }
    true
}
/// This is only used for serialize
pub const fn is_metadata_hidden(metadata: &Option<RawMetadata>) -> bool {
    if metadata.is_none() {
        return true;
    }
    if let Some(metadata) = metadata {
        return metadata.is_hidden();
    }
    false
}

/// This is only used for serialize
pub const fn is_zero_i32(value: Option<i32>) -> bool {
    if let Some(value) = value {
        return value == 0;
    }
    true
}
/// This is only used for serialize
pub fn is_zero_f32(value: Option<f32>) -> bool {
    if let Some(value) = value {
        return value == 0.0;
    }
    true
}

/// This is only used for serialize
pub const fn is_500_u32(value: Option<u32>) -> bool {
    if let Some(value) = value {
        return value == 500;
    }
    true
}
/// This is only used for serialize
pub const fn is_50_u32(value: Option<u32>) -> bool {
    if let Some(value) = value {
        return value == 50;
    }
    true
}
/// This is only used for serialize
pub const fn is_3_u32(value: Option<u32>) -> bool {
    if let Some(value) = value {
        return value == 3;
    }
    true
}
/// This is only used for serialize
pub const fn is_zero_u32(value: Option<u32>) -> bool {
    if let Some(value) = value {
        return value == 0;
    }
    true
}
