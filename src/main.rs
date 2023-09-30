use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
                println!("new connection accepted");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
