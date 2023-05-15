use clap::{Parser, Subcommand};
use std::time::Instant;
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ship {},
    Link {},
    Init { name: String },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ship {}) => {
            println!("New Deployment trigged instantly 🚀")
        }
        Some(Commands::Link {}) => {
            let before = Instant::now();
            println!("Linking with Github Repository...");

            println!("Linked to Nothing in {:.2?}", before.elapsed());
        }
        Some(Commands::Init { name }) => {
            let before = Instant::now();

            println!("Creating Project...");
            create_dir_all(name).unwrap_or_else(|e| panic!("Error creating dir: {}", e));

            println!("Created nothing in {} Successfully", name);
            println!("Took {:.2?}", before.elapsed());
        }
        None => {}
    }
}
