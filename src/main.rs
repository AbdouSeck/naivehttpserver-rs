use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;
use std::time::Duration;

use httpserver::ThreadPool;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let (status, html) = if is_valid(&request) {
        if is_get(&request) || is_post(&request) {
            if request.contains(" /sleep ") {
                thread::sleep(Duration::from_millis(5000));
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("hello.html").unwrap(),
                )
            } else {
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("hello.html").unwrap(),
                )
            }
        } else {
            (
                "HTTP/1.1 405 Method Not Allowed\r\n\r\n",
                fs::read_to_string("405.html").unwrap(),
            )
        }
    } else {
        (
            "HTTP/1.1 404 Not Found\r\n\r\n",
            fs::read_to_string("404.html").unwrap(),
        )
    };
    let response = format!("{}{}", status, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response: {}", status);
}

fn is_get(req: &str) -> bool {
    req.starts_with("GET /")
}

fn is_post(req: &str) -> bool {
    req.starts_with("POST /")
}

fn is_valid(req: &str) -> bool {
    match req.split("\r\n").nth(0) {
        Some(chunk) => chunk.contains(" / ") || chunk.contains(" /sleep "),
        None => false,
    }
}

fn main() {
    let url = "127.0.0.1:7878";
    let listener = TcpListener::bind(url).unwrap();
    let pool = match ThreadPool::new(5) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    println!("Serving your application at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_stream(stream);
        });
    }
}
