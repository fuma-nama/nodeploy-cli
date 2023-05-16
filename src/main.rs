use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
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
    Init { name: Option<String> },
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
            let target = name.to_owned().unwrap_or_else(|| read_line("Project Name"));
            let before = Instant::now();

            println!("Creating Project...");
            create_dir_all(&target).unwrap_or_else(|e| panic!("Error creating dir: {}", e));

            println!("Created nothing in {} Successfully", &target);
            println!("Took {:.2?}", before.elapsed());
        }
        None => {}
    }
}

fn read_line(message: &str) -> String {
    let mut target = String::new();
    print!("{}: ", message);
    stdout().flush().expect("Failed to read input");
    stdin()
        .read_line(&mut target)
        .expect("Failed to read input");

    return target.trim().to_string();
}
