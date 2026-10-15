use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("listening on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("request error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let request_line = {
        let mut buf_reader = BufReader::new(&stream);
        let mut line = String::new();

        buf_reader.read_line(&mut line)?;

        line.trim_end_matches(['\r', '\n']).to_string()
    };

    let (status, file) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "www/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "www/404.html")
    };

    let content = fs::read_to_string(file)?;

    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status, length, content
    );

    stream.write_all(response.as_bytes())?;

    Ok(())
}
