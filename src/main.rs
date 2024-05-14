use std::error::Error;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
struct Arguments {
    #[command(subcommand)]
    cmd: Commands,
}    

#[derive(Subcommand, Debug)]
enum Commands {
    Client,
    Server
}

mod client;
mod server;

fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
        match args.cmd {
        Commands::Client => {
            client::init("127.0.0.1", 2222);
        }
        Commands::Server => {
           server::listen("127.0.0.1", 2222);
        }
    }
    Ok(())
}
