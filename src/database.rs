use std::fmt::Display;
use std::path::{Path, PathBuf};
use rusqlite::{Connection, params};

use crate::book::Book;
use crate::config;
use crate::config::ConfigurationError;


const DATABASE_PATH: &str = "./books.db";
const ALL_BOOKS_QUERY: &str = "SELECT book_id, title, author FROM books";


pub(crate) fn list_books() -> Result<String, DatabaseError> {
    let conn = connect()?;
    let mut books: Vec<Book> = query_books(&conn)?;
    books.sort();
    let books: Vec<String> = books.iter().map(|b| b.to_string()).collect();
    Ok(books.join("\n"))
}

pub(crate) fn add_book(book: &Book, path: &String) -> Result<(), DatabaseError> {
    // get path and check if it exists
    let path = Path::new(&path);
    if !path.exists() { return Err(DatabaseError::FileNotFound(path.to_string_lossy().to_string())) }

    let conn = connect()?;

    // check if book already exists
    let books = query_books(&conn)?;
    if books.contains(&book) {
        return Err(DatabaseError::BookAlreadyExists(book.clone()))
    }

    // add book to database
    insert_book(&conn, &book)
}

fn query_books(conn: &Connection) -> Result<Vec<Book>, DatabaseError> {
    let mut stmt = match conn.prepare(ALL_BOOKS_QUERY) {
        Ok(stmt) => stmt,
        Err(e) => return Err(DatabaseError::ErrorGettingBooks(e))
    };
    let book_map = match stmt.query_map(params![], |row| {
        Ok(Book {
            title: row.get(1)?,
            author: row.get(2)?,
        })
    }) {
        Ok(book_map) => book_map,
        Err(e) => return Err(DatabaseError::ErrorGettingBooks(e))
    };
    let mut books: Vec<Book> = Vec::new();
    for book in book_map {
        let book = match book {
            Ok(book) => book,
            Err(e) => return Err(DatabaseError::ErrorGettingBooks(e))
        };
        books.push(book);
    }
    Ok(books)
}


pub(crate) fn get_book_path(book: &Book) -> Result<PathBuf, DatabaseError> {
    let mut path: PathBuf = match config::get_library_path() {
        Ok(path) => path,
        Err(e) => return Err(DatabaseError::ConfigError(e))
    };
    path.push(book.file_name());
    Ok(path)
}


fn insert_book(conn: &Connection, book: &Book) -> Result<(), DatabaseError> {
    let mut stmt = match conn.prepare("INSERT INTO books (title, author) VALUES (?,?)") {
        Ok(stmt) => stmt,
        Err(e) => return Err(DatabaseError::ErrorAddingBook(e))
    };
    match stmt.execute(&[&book.title, &book.author]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::ErrorAddingBook(e))
    }
}


fn connect() -> Result<Connection, DatabaseError> {
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
        Err(e) => { return Err(DatabaseError::ErrorCreatingTable(e)); }
    };
    Ok(conn)
}


pub(crate) enum DatabaseError {
    BookAlreadyExists(Book),
    ErrorAddingBook(rusqlite::Error),
    ErrorCreatingTable(rusqlite::Error),
    ErrorGettingBooks(rusqlite::Error),
    ErrorOpeningTable(rusqlite::Error),
    FileNotFound(String),
    ConfigError(ConfigurationError),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::ErrorOpeningTable(e) => write!(f, "Failed to open table: {}", e),
            DatabaseError::ErrorCreatingTable(e) => write!(f, "Failed to create table: {}", e),
            DatabaseError::ErrorGettingBooks(e) => write!(f, "Failed to get books: {}", e),
            DatabaseError::FileNotFound(path) => write!(f, "File not found: {}", path),
            DatabaseError::BookAlreadyExists(book) => write!(
                f, "Book already exists: {}", book.to_string()
            ),
            DatabaseError::ErrorAddingBook(e) => write!(f, "Failed to add book: {}", e),
            DatabaseError::ConfigError(e) => write!(f, "ConfigurationError: {}", e),
        }
    }
}
