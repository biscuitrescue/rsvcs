use std::{fs::{create_dir_all, write, read}, path::Path};
use clap::{Parser, Subcommand};
// use sha1::{Sha1, Digest};
use sha1::Sha1;

#[derive(Parser)] // auto impl trait
#[command(name = "rsvcs")]
#[command(about = "VCS in RS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { file: String },
    Commit {
        #[arg(short = 'm', long = "message")]
        message: String,
    },
    Init,
    Log,
}

fn init_repo() {
    let root = Path::new(".rsvcs");
    if root.exists() {
        println!("Repo already initialised!");
        return;
    }

    create_dir_all(root.join("commits"))
        .expect("Failed to create {root}/commits/");
    write(root.join("index"), b"")
        .expect("Failed to create {root}/index");
    write(root.join("HEAD"), b"")
        .expect("Failed to create {root}/HEAD");

    println!("Repo Initialised successfully");
}

fn add_file(file: &str) {
    let path = Path::new(file);

    if !path.exists() {
        eprintln!("File not found: {file}");
        return;
    }
    let contents = read(path)
        .expect("Failed to read {file}");

    let mut hasher = Sha1::new();
    hasher.update(&contents);

    // let hash = hex::encode(hasher.finalize);
}

fn commit(message: &str) {
    println!("Committing w msg: {}", message);
}
 
fn show_log() {
    println!("Showing commit log");
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
