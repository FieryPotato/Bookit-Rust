mod config;
mod database;
mod book;
mod ereader;

use std::fs;
use std::path::{PathBuf};
use clap::{Args, Parser, Subcommand};

use crate::config::ConfigItem;
use crate::book::Book;
use crate::ereader::load;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::List) => match database::list_books() {
            Ok(books) => {println!("{}", books);}
            Err(e) => {println!("Failed due to: {}", e);}
        },

        Some(Commands::Add(args)) => add_book(args),

        // remove(path)
        Some(Commands::Remove(args)) => println!("Removing book with title {}", args.title),

        Some(Commands::Load(args)) => match load(&Book {
            title: args.title, author: args.author
        }) {
            Ok(_) => println!("Successfully added book to ereader"),
            Err(e) => println!("Failed due to: {}", e)
        },

        // unload(path)
        Some(Commands::Unload(args)) => {
            println!("Removing book with title {} from ereader", args.title)
        }

        Some(Commands::Config(config)) => {
            println!("Configuring {} to {}", &config.target, &config.value);
            match config::configure(config.target, config.value) {
                Ok(_) => println!("Success"),
                Err(e) => println!("Failed due to: {}", e),
            };
        }

        // help()
        None => println!("No command specified"),
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,               // List books in database
    Add(AddArgs),       // Add book to database & device
    Remove(RemoveArgs), // Remove book from database
    Load(LoadArgs),     // add book from database to ereader
    Unload(UnloadArgs), // remove book from ereader
    Config(ConfigArgs), // configure app => Config(item, value)
}

#[derive(Args)]
struct AddArgs {
    title: String,
    author: String,
    path: String,
    #[arg(default_value_t = false)]
    install: bool,
}

#[derive(Args)]
struct RemoveArgs {
    title: String,
    #[arg(default_value_t = true)]
    uninstall: bool,
}

#[derive(Args)]
struct LoadArgs {
    title: String,
    author: String
}

#[derive(Args)]
struct UnloadArgs {
    title: String,
}

#[derive(Args)]
struct ConfigArgs {
    target: ConfigItem,
    value: String,
}


fn add_book(args: AddArgs) {
    println!(
        "Adding book with title {} and author {}, from source {}",
        args.title, args.author, args.path
    );
    let book: Book = Book { title: args.title, author: args.author };

    // move book to library
    let mut library_path: PathBuf = match config::get_library_path() {
        Ok(path) => path,
        Err(e) => {
            println!("Failed due to: {}", e);
            return;
        }
    };
    library_path.push(book.file_name());
    match fs::copy(&PathBuf::from(&args.path), &library_path) {
        Ok(_) => {},
        Err(e) => println!("Failed due to: {}", e),
    }

    // add book to database
    match database::add_book(&book, &args.path) {
        Ok(_) => println!("Added {} to library", &book.title),
        Err(e) => {println!("Failed due to: {}", e);}
    };
    if args.install {
        match load(&book) {
            Ok(_) => println!("{} added to ereader", &book.title),
            Err(e) => println!("Failed due to: {}", e)
        }
    }
}
