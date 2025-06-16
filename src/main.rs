mod commands;
mod init;
mod add;
mod commit;

use commands::{Cli, Commands};
use clap::Parser;

// TODO: Fix err handling -> anyhow

fn show_log() {
    println!("Showing commit log");
}

fn main() {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::init_repo(),
        Commands::Add { file } => add::add_file(&file),
        Commands::Commit { message } => commit::commit_changes(&message),
        Commands::Log => show_log(),
    }
}
