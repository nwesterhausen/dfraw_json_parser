lazy_static::lazy_static! {
    // The file encoding supported by dwarf fortress
    pub static ref DF_ENCODING: &'static encoding_rs::Encoding =
        encoding_rs::Encoding::for_label(b"latin1").unwrap();
}
