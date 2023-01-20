use std::path::{Path, PathBuf};

use crate::parser::raws::{
    dimensions::Dimensions, info_txt::DFInfoFile, DFRawCommon, RawObjectKind,
};

impl super::DFTilePage {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            raw_header: DFRawCommon::from(id, raw, info_text, RawObjectKind::GraphicsTilePage),
            file: PathBuf::new(),
            page_dim: Dimensions::zero(),
            tile_dim: Dimensions::zero(),
        }
    }
    pub fn set_file(&mut self, file: &str) {
        self.file = file.split('/').collect();
    }
    pub fn set_tile_dim_from_token(&mut self, token: &str) {
        self.tile_dim = Dimensions::from_token_xy(token);
    }
    pub fn set_page_dim_from_token(&mut self, token: &str) {
        self.page_dim = Dimensions::from_token_xy(token);
    }
    pub fn get_raw_header(&self) -> &DFRawCommon {
        &self.raw_header
    }
    pub fn get_tile_dim(&self) -> Dimensions {
        self.tile_dim
    }

    pub fn get_page_dim(&self) -> Dimensions {
        self.page_dim
    }
    pub fn get_file_path(&self) -> String {
        let mut images_path = self.raw_header.dfraw_found_in.get_path();
        images_path.push(self.raw_header.get_dfraw_relative_path());
        images_path.push("graphics");
        images_path.push(self.file.as_path());

        images_path.into_os_string().to_string_lossy().to_string()
    }
}
