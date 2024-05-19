use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

async fn handle_connection(conn: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer: String = format!("{:?}",conn.peer_addr().expect("Bad connection"));
    let (mut reader, mut writer) = io::split(conn);
    println!("Host {:?} has connected", peer);

    loop {
        let mut data = vec![0; 1024];
        match reader.read(&mut data[..]).await? {
            0 => {
                break
            }
            _ => {
                let incoming =  String::from_utf8(data).expect("Invalid Bytes");
                println!("Read : {}", incoming);
                writer.write_all(&incoming.into_bytes()).await?;
            }
        }
    }
    println!("Host {:?} has disconnected", peer);
    Ok(())
}

#[tokio::main]
pub async fn listen(addr: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let listen_address: String = format!("{}:{}", addr, port);
    println!(
        "listening started on {}, ready to accept incoming traffic",
        &listen_address
    );
    let listener = TcpListener::bind(&listen_address).await.expect("Error while binding");
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket).await.expect("Could not handle incoming connection...");
        });
    }
}
