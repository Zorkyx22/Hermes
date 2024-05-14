use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

async fn handle_connection(mut conn: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer: String = format!("{:?}",conn.peer_addr().expect("Bad connection"));
    let (mut reader, mut writer) = io::split(conn);
    let mut debug_output = io::stdout();
    debug_output.write_all(&format!("Host {:?} has connected", peer).into_bytes());

    loop {
        let mut data = vec![0; 1024];
        match reader.read(&mut data[..]).await? {
            0 => {
                debug_output.write_all(&format!("Read nothing. Breaking").into_bytes());
                break
            }
            _ => {
                debug_output.write_all(&format!("Read : {}", String::from_utf8(data).expect("Invalid Bytes")).into_bytes());
            }
        }
    }
    debug_output.write_all(&format!("Host {:?} has disconnected", peer).into_bytes());
    Ok(())
}

#[tokio::main]
pub async fn listen(addr: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let listen_address: String = format!("{}:{}", addr, port);
    println!(
        "listening started on {}, ready to accept incoming traffic",
        &listen_address
    );
    let listener = TcpListener::bind(&listen_address).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket);
        });
    }
}
