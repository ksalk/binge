use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli;

fn main() {
    let _ = Cli::parse();
}
