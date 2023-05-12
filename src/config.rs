use serde_json::{Error, json};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use crate::ConfigItem;

const CONFIG_FILE: &str = "config.json";

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

fn get_config() -> Result<Configuration, ConfigurationError> {
    let config_file: File = get_config_file()?;
    let buffer: BufReader<File> = BufReader::new(config_file);
    let config: Configuration = serde_json::from_reader(buffer)?;

}

fn get_config_file() -> Result<File, ConfigurationError> {
    match File::open(Path::new(CONFIG_FILE)) {
        Ok(file) => Ok(file),
        // If there is no file, we create it and try again.
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                create_config_file()?;
                match get_config_file() {
                    Ok(file) => Ok(file),
                    Err(e) => Err(e),
                }
            } else {
                return Err(ConfigurationError::IoError(e));
            }
        }
    }
}

fn create_config_file() -> Result<(), ConfigurationError> {
    let mut settings = match File::create(Path::new(CONFIG_FILE)) {
        Ok(file) => file,
        Err(e) => return Err(ConfigurationError::IoError(e)),
    };
    let default_settings = json!({
        "ebook_path": "",
        "library_path": ""
    }).to_string();
    match settings.write((&default_settings).as_ref()) {
        Ok(_) => Ok(()),
        Err(e) => Err(ConfigurationError::IoError(e)),
    }
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    ebook_path: PathBuf,
    library_path: PathBuf,
}

pub(crate) enum ConfigurationError {
    IoError(std::io::Error),
    SerdeError(Error),
}
