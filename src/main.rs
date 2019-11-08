use std::net::TcpListener;
use std::process;

use httpserver::{parsers, ThreadPool};

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
            parsers::handle_stream(stream);
        });
    }
}
