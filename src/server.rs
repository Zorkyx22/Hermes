use std::error::Error;
use std::net::TcpListener;
use std::thread;
use std::io::Write;

pub fn listen(addr: &str, port: u16) -> Result<bool, Box<dyn Error>> {
    let listen_address: String = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(listen_address.clone()).expect("invalid address or port...");
    println!("listening started on {}, ready to accept incoming traffic", listen_address.clone());
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.expect("Error when trying to handle incoming connection");
            stream.write(b"Hello World\r\n").unwrap();
        });
    }
    Ok(true)
}