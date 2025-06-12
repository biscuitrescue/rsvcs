use std::path::Path;
use std::fs::{create_dir_all, write};
use clap::{Parser, Subcommand};

#[derive(Parser)] // auto impl trait
#[command(name = "rsvcs")]
#[command(about = "VCS in RS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { file: String },
    Commit {
        #[arg(short = 'm', long = "message")]
        message: String,
    },
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_repo(),
        Commands::Add { file } => add_file(&file),
        Commands::Commit { message } => commit(&message),
        Commands::Log => show_log(),
    }
}

fn init_repo() {
    if Path::new(".rsvcs").exists() {
        println!("Repo already initialised!");
        return;
    }

    create_dir_all(".rsvcs/commits").unwrap();
    write(".rsvcs/index", "").unwrap();
    write(".rsvcs/HEAD", "").unwrap();

    println!("Repo Initialised successfully");
}

fn add_file(file: &str) {
    println!("Adding file: {}", file);
}

fn commit(message: &str) {
    println!("Committing w msg: {}", message);
}
 
fn show_log() {
    println!("Showing commit log");
}
