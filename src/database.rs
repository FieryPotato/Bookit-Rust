use std::fmt::Display;
use rusqlite::{Connection, params};

use crate::book::Book;


const DATABASE_PATH: &str = "./books.db";
const ALL_BOOKS_QUERY: &str = "SELECT book_id, title, author FROM books";


pub(crate) fn list_books() -> Result<String, DatabaseError> {
    let mut books: Vec<Book> = match get_books() {
        Ok(books) => books,
        Err(e) => return Err(e)
    };
    books.sort();
    Ok(books.join("\n"))
}


fn get_books() -> Result<Vec<Book>, DatabaseError> {
    let conn = match Connection::open(DATABASE_PATH) {
        Ok(c) => c,
        Err(e) => {
            return Err(DatabaseError::ErrorOpeningTable(e));
        }
    };
    match conn.execute("create table if not exists books (
        id integer primary key, title text not null, author text not null
        )", params![],
    ) {
        Ok(_) => (),
        Err(e) => { return Err(DatabaseError::ErrorOpeningTable(e)); }
    };
    match query_books(&conn) {
        Ok(books) => Ok(books),
        Err(e) => Err(DatabaseError::ErrorGettingBooks(e)),
    }
}

fn query_books(conn: &Connection) -> Result<Vec<Book>, rusqlite::Error> {
    let mut stmt = conn.prepare(ALL_BOOKS_QUERY)?;
    let book_map = stmt.query_map(params![], |row| {
        Ok(Book {
            title: row.get(1)?,
            author: row.get(2)?,
        })
    })?;
    let mut books: Vec<Book> = Vec::new();
    for book in book_map {
        books.push(book?);
    }
    Ok(books)
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
