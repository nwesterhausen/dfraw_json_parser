mod header;
mod module_info;
mod ranges;

pub fn parse_raw_file<P: AsRef<Path>>(full_path: &P) -> DFRaw {
    let raw_type = RawObjectKind::from_raw_file_path(full_path);
    if raw_type = RawObjectKind::None {
        log::warn!("DFRaw - Unable to determine raw type for {:?}", full_path);
        return DFRaw::empty();
    }

    let raw_module_info = ModuleInfoFile::from_raw_file_path(full_path);
}
