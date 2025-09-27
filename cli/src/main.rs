mod command;

use clap::Parser;
use command::{Commands, create_pwa};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name, url, out } => {
            create_pwa(name, url, out.clone());
        }
    }
}
