use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn client_handler(mut stream: TcpStream) {
    let stream_buff = BufReader::new(&mut stream);
    let requests: Vec<_> = stream_buff
        .lines()
        .map(|r| r.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let start_line = requests.get(0).expect("Unable to read start line.");

    let words: Vec<&str> = start_line.split(" ").collect();
    let path = words[1];

    let data;
    if path == "/" {
        data = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    } else {
        data = "HTTP/1.1 400 Not Found\r\n\r\n".to_string();
    }

    stream
        .write(data.as_bytes())
        .expect("Unable to write to stream.");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("new connection accepted");
                client_handler(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
