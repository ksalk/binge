mod commands;
mod db;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search { name: String },
    Add { name: String },
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { name } => commands::search(&name)?,
        Commands::Add { name } => commands::add(&name)?,
        Commands::Clear => commands::clear()?,
    }

    Ok(())
}
