use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;
use std::time::Duration;

use httpserver::{parsers, ThreadPool};

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let contents = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", contents);
    let request = parsers::parse_request(&contents);
    let (status, html) = if is_valid(&request) {
        if is_get(&request) || is_post(&request) {
            if is_sleep(&request) {
                thread::sleep(Duration::from_millis(5000));
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("html/sleep.html").unwrap(),
                )
            } else {
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("html/hello.html").unwrap(),
                )
            }
        } else {
            (
                "HTTP/1.1 405 Method Not Allowed\r\n\r\n",
                fs::read_to_string("html/405.html").unwrap(),
            )
        }
    } else {
        (
            "HTTP/1.1 404 Not Found\r\n\r\n",
            fs::read_to_string("html/404.html").unwrap(),
        )
    };
    let response = format!("{}{}", status, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response: {}", status);
    println!("Body: {}", html);
}

fn is_get(req: &HashMap<String, String>) -> bool {
    req.get("method").unwrap().contains("GET")
}

fn is_post(req: &HashMap<String, String>) -> bool {
    req.get("method").unwrap().contains("POST")
}

fn is_valid(req: &HashMap<String, String>) -> bool {
    let endpoint = req.get("endpoint").unwrap();
    endpoint == "/" || endpoint.contains("/sleep")
}

fn is_sleep(req: &HashMap<String, String>) -> bool {
    let endpoint = req.get("endpoint").unwrap();
    endpoint.contains("/sleep") || endpoint.contains("/sleep/")
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
