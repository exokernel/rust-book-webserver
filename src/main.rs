use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use rust_book_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let ip = stream.peer_addr().unwrap().ip().to_string();

    let (status_line, contents) = if buffer.starts_with(get) {
        println!("GET /");
        ("HTTP/1.1 200 OK", format!("{}\r\n", ip))
    } else if buffer.starts_with(sleep) {
        println!("GET /sleep");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "slept 5 seconds".to_string())
    } else {
        println!("404");
        ("HTTP/1.1 404 NOT FOUND", "404'd!".to_string())
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
