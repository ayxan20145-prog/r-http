use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("listening on http://localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, file) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };
    let content = fs::read_to_string(file).unwrap();
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, content
    );

    stream.write_all(response.as_bytes()).unwrap();
}
