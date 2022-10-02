use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
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
}

fn handle_connection(mut stream: TcpStream) {
    let ip = stream.peer_addr().unwrap().ip().to_string();
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);


    let status_line = "HTTP/1.1 200 OK";
    let contents = format!("{}\r\n", ip);
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", ip);

    stream.write_all(response.as_bytes()).unwrap();
}
