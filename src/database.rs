use std::fmt::Display;
use rusqlite::{Connection, Result, NO_PARAMS};

use crate::book::Book;


const DATABASE_PATH: &str = "./books.db";
const ALL_BOOKS: &str = "SELECT book_id, title, author FROM books";


fn get_books() -> Result<Vec<Book>, DatabaseError> {
    let conn = match Connection::open(DATABASE_PATH) {
        Ok(c) => c,
        Err(e) => {
            return Err(DatabaseError::ErrorOpeningTable(e));
        }
    };
    match conn.execute("create table if not exists books (
        id integer primary key, title text not null, author text not null
        )", NO_PARAMS,
    ) {
        Ok(_) => (),
        Err(e) => {
            return Err(DatabaseError::ErrorOpeningTable(e));
        }
    };
    let stmt = match conn.prepare(ALL_BOOKS) {
        Ok(s) => s,
        Err(e) => {
            return Err(DatabaseError::ErrorGettingBooks(e));
        }
    };
    let books = stmt.query_map(NO_PARAMS, |row| {
        Ok()
    })

    Ok(())
}


pub(crate) enum DatabaseError {
    ErrorOpeningTable(rusqlite::Error),
    ErrorCreatingTable(rusqlite::Error),
    ErrorGettingBooks(rusqlite::Error),
}
impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ErrorOpeningTable(e) => write!(f, "Failed to open table: {}", e),
            DatabaseError::ErrorCreatingTable(e) => write!(f, "Failed to create table: {}", e),
            DatabaseError::ErrorGettingBooks(e) => write!(f, "Failed to get books: {}", e),
        }
    }
}
