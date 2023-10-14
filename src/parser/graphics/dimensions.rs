use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct Dimensions {
    x: i32,
    y: i32,
}

#[allow(dead_code)] // Until we add graphics parsing
impl Dimensions {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_token(token: &str) -> Self {
        let split = token.split(':').collect::<Vec<&str>>();
        //	[TILE_DIM:32:32]

        let Some(dim_x) = split.first() else {
            log::error!(
                "Missing required number of tokens for Dimensions! {}",
                token
            );
            return Self { x: 0, y: 0 };
        };
        let Some(dim_y) = split.get(1) else {
            log::error!(
                "Missing required number of tokens for Dimensions! {}",
                token
            );
            return Self { x: 0, y: 0 };
        };

        let x: i32 = match dim_x.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as Dimensions:x, {:?}", token, e);
                0
            }
        };
        let y: i32 = match dim_y.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as Dimensions:y, {:?}", token, e);
                0
            }
        };

        Self { x, y }
    }
    pub fn empty() -> Self {
        Dimensions::zero()
    }
    pub fn new() -> Self {
        Dimensions::zero()
    }
    pub fn is_default(self) -> bool {
        self.x == 0 && self.y == 0
    }
    /// Used in serialization
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_empty(&self) -> bool {
        self.is_default()
    }
}
