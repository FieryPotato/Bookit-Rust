use std::path::PathBuf;
use crate::ConfigItem;

pub(crate) fn configure(target: ConfigItem, value: String) -> Result<(), ConfigurationError> {
    let path: PathBuf = value.into();
    match target {
        ConfigItem::EbookPath => set_ebook_path(path),
        ConfigItem::LibraryPath => set_library_path(path),
    }
}

fn set_ebook_path(value: PathBuf) -> Result<(), ConfigurationError> {

    Ok(())
}

fn set_library_path(value: PathBuf) -> Result<(), ConfigurationError> {
    // TODO
    Ok(())
}

pub(crate) enum ConfigurationError {}
