lazy_static::lazy_static! {
    /// The encoding used by dwarf fortress text files
    pub static ref DF_ENCODING: &'static encoding_rs::Encoding =
        encoding_rs::Encoding::for_label(b"latin1").unwrap_or_else(|| {
            panic!("Failed to get encoding: latin1");
        });
}
