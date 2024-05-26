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
    Client {
        #[clap(short='a', long)]
        addr: String,
        #[clap(short='u', long)]
        username: String,
    },
    Server {
        #[clap(short='a', long)]
        addr: String,
    }
}

mod client;
mod server;
mod app;
mod screens;
mod datatypes;

fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
        match args.cmd {
        Commands::Client {addr, username} => {
            let _ = client::init(addr, username);
        }
        Commands::Server {addr} => {
           let _ = server::listen(addr);
        }
    }
    Ok(())
}
