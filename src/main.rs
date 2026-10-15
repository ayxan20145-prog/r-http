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
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("src/hello.html").unwrap();
        let length = content.len();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status, length, content
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("src/404.html").unwrap();
        let length = content.len();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status, length, content
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
