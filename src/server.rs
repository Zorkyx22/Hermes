use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::sync::{Mutex}; 
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

// I want to make an object to represent a connection, with a method for listening, sending, and
// sharing all messages. I will probably create a HashMap containing all objects like in the
// example listed in the readme.
struct Chatroom {
    members: HashMap<SocketAddr, WriteHalf<TcpStream>>,
}
impl Chatroom {
    fn new() -> Self {
        Chatroom {
            members: HashMap::new(),
        }
    }

    async fn broadcast(&mut self, message: String) {
        for member in self.members.iter_mut() {
            println!("{}", message.clone());
            member.1.write_all(message.as_bytes()).await.expect("Broadcasting error");
        }
    }

}

#[tokio::main]
pub async fn listen(listen_address: String) -> Result<(), Box<dyn Error>> {
    println!(
        "listening started on {}, ready to accept incoming traffic",
        &listen_address
    );
    let listener = TcpListener::bind(&listen_address).await.expect("Error while binding");
    let chatroom = Arc::new(Mutex::new(Chatroom::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        let chatroom = Arc::clone(&chatroom);
        tokio::spawn(async move {
            handle_connection(chatroom, socket).await.expect("Could not handle incoming connection...");
        });
    }
}


// Handle incoming connections. This is a dumb implementation. I will make an OOP implementation
// next.
async fn handle_connection(room: Arc<Mutex<Chatroom>>, conn: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer: SocketAddr = conn.peer_addr().expect("Bad connection");
    let (mut reader, writer) = io::split(conn);
    let mut room_lock = room.lock().await;
    room_lock.members.insert(peer.clone(), writer);
    drop(room_lock);
    
    println!("Host {:?} has connected", &peer);

    loop {
        let mut data = vec![0; 1024];
        match reader.read(&mut data[..]).await? {
            0 => {
                break
            }
            _ => {
                let incoming =  String::from_utf8(data).expect("Invalid Bytes");
                room.lock().await.broadcast(incoming).await;
            }
        }
    }

    let mut room_lock = room.lock().await;
    room_lock.members.remove(&peer);
    drop(room_lock);
    
    println!("Host {:?} has disconnected", &peer);
    Ok(())
}

