mod config;

use clap::{Args, Parser, Subcommand, ValueEnum};

use std::fmt::{Display, Formatter};
use std::fmt;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        // list()
        Some(Commands::List) => println!("Listing books:"),

        // add(path)
        Some(Commands::Add(path)) => println!(
            "Adding book with title {} and author {}, from source {}",
            path.title, path.author, path.path
        ),

        // remove(path)
        Some(Commands::Remove(path)) => println!("Removing book with title {}", path.title),

        // load(path)
        Some(Commands::Load(path)) => println!("Adding book with title {} to ereader", path.title),

        // Unload(path)
        Some(Commands::Unload(path)) => {
            println!("Removing book with title {} from ereader", path.title)
        }

        // Config(config)
        Some(Commands::Config(config)) => {
            println!("Configuring {} to {}", config.target, config.value)
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

#[derive(Clone, ValueEnum)]
pub(crate) enum ConfigItem {
    EbookPath,
    LibraryPath,
}

impl Display for ConfigItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConfigItem::EbookPath => write!(f, "Ebook Path"),
            ConfigItem::LibraryPath => write!(f, "Library Path"),
        }
    }
}
