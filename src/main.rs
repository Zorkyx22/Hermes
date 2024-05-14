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
    Server,
    Getout
}

mod client;
mod server;
mod fuckit;

fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
        match args.cmd {
        Commands::Client => {
            client::init("127.0.0.1", 2222);
        }
        Commands::Server => {
           server::listen("127.0.0.1", 2222);
        }
        Commands::Getout => {
            fuckit::go("127.0.0.1:2222");
        }
    }
    Ok(())
}
