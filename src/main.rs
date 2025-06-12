mod commands;
mod repo;
mod objects;

use commands::{Cli, Commands};
use clap::Parser;

// TODO: Fix err handling -> anyhow

fn commit(message: &str) {
    println!("Committing w msg: {}", message);
}
 
fn show_log() {
    println!("Showing commit log");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => repo::init_repo(),
        Commands::Add { file } => objects::add_file(&file),
        Commands::Commit { message } => commit(&message),
        Commands::Log => show_log(),
    }
}
