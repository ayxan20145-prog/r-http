use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("listening on http://localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request: {:#?}", http_request);

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("src/hello.html").unwrap();
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, content
    );

    stream.write_all(response.as_bytes()).unwrap();
}
