use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{PathBuf};
use crate::book::Book;
use crate::{config, database};
use crate::config::ConfigurationError;
use crate::database::DatabaseError;

pub(crate) fn load(book: &Book) -> Result<(), EreaderError> {
    let ereader_path: PathBuf = match config::get_ereader_path() {
        Ok(path) => path,
        Err(ConfigurationError::NoEreaderPath) => return Err(EreaderError::NoEreaderPath),
        Err(e) => return Err(EreaderError::ConfigError(e))
    };
    if !ereader_path.exists() { return Err(EreaderError::EreaderNotConnected) };
    let book_path: PathBuf = match database::get_book_path(book) {
        Ok(path) => path,
        Err(e) => return Err(EreaderError::DatabaseError(e))
    };
    match fs::copy(book_path, ereader_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(EreaderError::FileSystemError(e))
    }
}

pub(crate) fn unload(book: &Book) -> Result<(), EreaderError> {
    let ereader_path: PathBuf = match config::get_ereader_path() {
        Ok(path) => path,
        Err(ConfigurationError::NoEreaderPath) => return Err(EreaderError::NoEreaderPath),
        Err(e) => return Err(EreaderError::ConfigError(e))
    };
    if !ereader_path.exists() { return Err(EreaderError::EreaderNotConnected) };
    let book_path: PathBuf = match database::get_book_path(book) {
        Ok(path) => path,
        Err(e) => return Err(EreaderError::DatabaseError(e))
    };
}


pub(crate) enum EreaderError {
    NoEreaderPath,
    ConfigError(ConfigurationError),
    EreaderNotConnected,
    FileSystemError(std::io::Error),
    DatabaseError(DatabaseError),
}
impl Display for EreaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EreaderError::NoEreaderPath => write!(f, "No Ereader path set."),
            EreaderError::ConfigError(e) => write!(f, "{}", e),
            EreaderError::EreaderNotConnected => write!(f, "Ereader not connected."),
            EreaderError::FileSystemError(e) => write!(f, "{}", e),
            EreaderError::DatabaseError(e) => write!(f, "{}", e),
        }

    }
}