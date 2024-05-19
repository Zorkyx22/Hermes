use std::{error::Error, io};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
pub async fn go(server_address: &str) -> Result<(), Box<dyn Error>>{
    let mut socket = TcpStream::connect(&server_address).await.expect("Failed to connect");
    socket.write_all("Bonjour!".as_bytes()).await?;
    loop {
        let mut peeked = [0; 10];
        let n_bytes_waiting = socket.peek(&mut peeked).await?;
        if n_bytes_waiting > 0 {
            let mut data = vec![0; 1024];
            socket.read(&mut data).await?;
            let message = std::str::from_utf8(&data[..]).expect("error parsing received message").to_string();
            println!("{}", message);
        };
        println!("Read {} bytes", n_bytes_waiting);
    }
    println!("done, exiting");
    Ok(())
}
