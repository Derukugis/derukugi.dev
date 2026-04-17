use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("connection error: {e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&stream);

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return Ok(()),
    };
        
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/index.html"),
        "GET /robots.txt HTTP/1.1" => ("HTTP/1.1 200 OK", "static/robots.txt"),
        "GET /sitemap.xml HTTP/1.1" => ("HTTP/1.1 200 OK", "static/sitemap.xml"),

        _ => { stream.write_all(b"HTTP/1.1 301 Moved Permanently\r\nLocation: /\r\n\r\n").unwrap();
        return Ok(())
        }
    };

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n");
            return Ok(());
        }
    };
    let length = contents.len();

    let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
 
    stream.write_all(response.as_bytes()).unwrap(); 

    Ok(())
}
