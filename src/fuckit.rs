use std::{error::Error, io};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
pub async fn go(server_address: &str) -> Result<(), Box<dyn Error>>{
    let mut socket = TcpStream::connect(&server_address).await.expect("Failed to connect");
    socket.write_all("Bonjour!".as_bytes()).await?;
    println!("done, exiting");
    Ok(())
}
