use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

const MAX_REQUESTS: usize = 2;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(e) => panic!("{}", e),
    };
    let pool = ThreadPool::build(4).expect("Should fail only if you give 0");

    for stream in listener.incoming().take(MAX_REQUESTS) {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }

    println!("Server is shutting down after {MAX_REQUESTS} requests");
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&stream);
    let request_line = buffer.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    // let _http_request: Vec<_> = buffer
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();
}
