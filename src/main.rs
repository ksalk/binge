mod api;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    List,
    Clear,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name } => api::search(&name),
        Commands::List => println!("list command"),
        Commands::Clear => println!("clear command"),
    };

    return Ok(());
}
