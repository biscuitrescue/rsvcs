use clap::{Parser, Subcommand};

#[derive(Parser)] // auto impl trait
#[command(name = "rsvcs")]
#[command(about = "VCS in RS", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { file: String },
    Commit { // TODO: shift to tree for commit deets
        #[arg(short = 'm', long = "message")]
        message: String,
    },
    Init,
    Log,
}
