use std::ffi::CString;

use clap::{Parser, Subcommand};

mod runtime;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        name: Option<String>,
        #[clap(long)]
        rootfs: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { name, rootfs } => {
            println!("`run` called with {name:?}");

            let cmd = CString::new("/bin/sh").unwrap();
            let args = [cmd.clone(), CString::new("-l").unwrap()].to_vec();

            runtime::runtime::run_process(cmd, args, rootfs.as_str());
        }
    }
}
