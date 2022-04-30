use clap::StructOpt;
mod commands;
mod types;
use lib::error::dgitError;
use std::path::PathBuf;
mod lib;
#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}
#[derive(clap::Subcommand)]
enum Action {
    #[clap(about = "Staging")]
    Add { path: PathBuf },
    #[clap(about = "Commit")]
    Commit {
        #[clap(short, long)]
        message: String,
    },
    #[clap(about = "Change username and password")]
    Config {
        #[clap(short, long)]
        username: String,
        #[clap(short, long)]
        email: String,
    },

    #[clap(about = "Undo prev Staging")]
    Restore { path: Option<PathBuf> },

    #[clap(about = "Revert to old commits ")]
    Switch { target: String },

    // #[clap(about = "")]
    // Help,
    #[clap(about = "Initialize before usage")]
    Init,

    #[clap(about = "Commits info")]
    Log,

    #[clap(about = "Staging info")]
    Status,
}
fn main() -> Result<(), dgitError> {
    let cli = Args::parse();
    match &cli.action {
        Action::Init => {
            commands::init()?;
        }
        Action::Add { path } => {
            commands::add(path)?;
        }
        Action::Commit { message } => commands::commit(message.to_owned())?,
        Action::Config { username, email } => {
            println!("Config")
        }
        Action::Restore { path } => {
            println!("Restore")
        }
        Action::Switch { target } => {
            println!("Switch")
        }
        Action::Init => {
            println!("Initialized git")
        }
        Action::Log => {
            println!("Log")
        }
        Action::Status => {
            println!("Status")
        }
    }
    Ok(())
}
