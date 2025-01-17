// Listener para conexões TCP.

use std::net::TcpListener;

#[allow(unused)]
pub fn start_tcp_listener(addr: &str) {
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Listening for TCP connections on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("New TCP connection established");
                // Processar a conexão TCP
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

