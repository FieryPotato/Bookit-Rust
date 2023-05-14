mod config;
mod database;
mod book;

use clap::{Args, Parser, Subcommand};

use config::ConfigItem;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::List) => match database::list_books() {
            Ok(books) => {println!("{}", books);}
            Err(e) => {println!("Failed due to: {}", e);}
        },

        // add(path)
        Some(Commands::Add(path)) => println!(
            "Adding book with title {} and author {}, from source {}",
            path.title, path.author, path.path
        ),

        // remove(path)
        Some(Commands::Remove(path)) => println!("Removing book with title {}", path.title),

        // load(path)
        Some(Commands::Load(path)) => println!("Adding book with title {} to ereader", path.title),

        // unload(path)
        Some(Commands::Unload(path)) => {
            println!("Removing book with title {} from ereader", path.title)
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
