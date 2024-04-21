use std::net::{TcpListener, TcpStream};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut players: Vec<TcpStream> = Vec::with_capacity(2);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Some(addr) = stream.peer_addr().ok() {
                    println!("Connection establish with client: {}", addr);
                }
                players.push(stream);
                
                if players.len() == 2 { break }
            }
            
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
