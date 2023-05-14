use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

const CONFIG_FILE: &str = "config.json";

fn get_config_item(item: ConfigItem) -> Result<String, ConfigurationError> {
    match item {
        ConfigItem::EreaderPath => Ok(get_config()?.ereader_path),
        ConfigItem::LibraryPath => Ok(get_config()?.library_path),
    }
}

pub(crate) fn configure(target: ConfigItem, value: String) -> Result<(), ConfigurationError> {
    match target {
        ConfigItem::EreaderPath => set_ereader_path(value),
        ConfigItem::LibraryPath => set_library_path(value),
    }
}

fn set_ereader_path(value: String) -> Result<(), ConfigurationError> {
    let mut config: Configuration = get_config()?;
    config.ereader_path = value;
    set_config(config)?;
    Ok(())
}

pub(crate) fn get_ereader_path() -> Result<PathBuf, ConfigurationError> {
    let path: String = get_config_item(ConfigItem::EreaderPath)?;
    if path.len() == 0 {
        return Err(ConfigurationError::NoEreaderPath)
    }
    Ok(PathBuf::from(path))
}

fn set_library_path(value: String) -> Result<(), ConfigurationError> {
    let mut config: Configuration = get_config()?;
    config.library_path = value;
    set_config(config)?;
    Ok(())
}

pub(crate) fn get_library_path() -> Result<PathBuf, ConfigurationError> {
    let path: String = get_config_item(ConfigItem::LibraryPath)?;
    if path.len() == 0 {
        return Err(ConfigurationError::NoLibraryPath)
    }
    Ok(PathBuf::from(path))
}

fn set_config(config: Configuration) -> Result<(), ConfigurationError> {
    let serilaized = match serde_json::to_string(&config) {
        Ok(s) => s,
        Err(e) => return Err(ConfigurationError::JSONError(e)),
    };
    // create a new file because the whole file should already be loaded
    // so there's no risk of losing data.
    match File::create(CONFIG_FILE) {
        Ok(mut file) => match file.write_all(serilaized.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => Err(ConfigurationError::IOError(e)),
        },
        Err(e) => Err(ConfigurationError::IOError(e)),
    }
}

fn get_config() -> Result<Configuration, ConfigurationError> {
    let config_file: File = get_config_file()?;
    let buffer: BufReader<File> = BufReader::new(config_file);
    match serde_json::from_reader(buffer) {
        Ok(config) => Ok(config),
        Err(e) => {
            return Err(ConfigurationError::JSONError(e));
        }
    }
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
                return Err(ConfigurationError::IOError(e));
            }
        }
    }
}

fn create_config_file() -> Result<(), ConfigurationError> {
    let mut settings = match File::create(Path::new(CONFIG_FILE)) {
        Ok(file) => file,
        Err(e) => return Err(ConfigurationError::IOError(e)),
    };
    let default_settings = json!({
        "ereader_path": "",
        "library_path": ""
    })
    .to_string();
    match settings.write((&default_settings).as_ref()) {
        Ok(_) => Ok(()),
        Err(e) => Err(ConfigurationError::IOError(e)),
    }
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    ereader_path: String,
    library_path: String,
}

pub(crate) enum ConfigurationError {
    IOError(std::io::Error),
    JSONError(Error),
    NoEreaderPath,
    NoLibraryPath,
}
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::IOError(e) => write!(f, "{}", e),
            ConfigurationError::JSONError(e) => write!(f, "{}", e),
            ConfigurationError::NoEreaderPath => write!(f, "No Ereader Path"),
            ConfigurationError::NoLibraryPath => write!(f, "No Library Path"),
        }
    }
}

#[derive(Clone, ValueEnum)]
pub(crate) enum ConfigItem {
    EreaderPath,
    LibraryPath,
}

impl Display for ConfigItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConfigItem::EreaderPath => write!(f, "Ereader Path"),
            ConfigItem::LibraryPath => write!(f, "Library Path"),
        }
    }
}
