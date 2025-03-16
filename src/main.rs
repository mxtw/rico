use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { name } => {
            println!("`run` called with {name:?}");
        }
    }
}
