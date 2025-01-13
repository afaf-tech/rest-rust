mod server;
mod cli;

use clap::{Parser, Subcommand};




#[derive(Parser, Debug)]
#[command(name = "Afaf REST Rust")]
#[command(about = "A Rust-based RESTful service", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the server
    Server,
    /// CLI-based functionality
    Cli(cli::CliArgs),
}

pub fn run() -> Result<(), std::io::Error> {
    let args = Args::parse();
    match args.command {
        Commands::Server => server::run_rest(),  // Ensure this returns a Result
        Commands::Cli(cli_args) => cli::run(cli_args), // Ensure this returns a Result
    }
}
