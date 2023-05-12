use clap::{Parser, Subcommand};


#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Add { path: String },
    Remove { title: String },
    Load { title: String },
    Unload { title: String },
    Config { config: Vec<String> },
}

#[derive(Subcommand, Debug)]
enum Configurable {
    Ereader { path: String },
    Books { path: String },
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::List) => println!("Listing books:"), // list(),
        Some(Commands::Add { path } ) => println!("adding book from path {}", path), // add(path),
        Some(Commands::Remove { title } ) => println!("removing book with title {}", title), // (title),
        Some(Commands::Load { title } ) => println!("Loading book with title {} to ereader", title), // load(title),
        Some(Commands::Unload { title } ) => println!("Removing book with title {} from ereader", title), // Unload(title),
        Some(Commands::Config { config } ) => println!("Configuring ereader with args {:?}", config), // Config(args),
        None => println!("No command specified"),
    }
}
