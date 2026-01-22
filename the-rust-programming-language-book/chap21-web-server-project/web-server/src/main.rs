use std:: {
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Hello, web-server!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

// handler for each incoming stream
fn handle_connection(mut stream: TcpStream) {
    let crlf = "\r\n";

    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    /*
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
    */

    let (status_line, html) = if request_line == "GET / HTTP/1.1" {
        (format!("HTTP/1.1 200 OK{crlf}"), "hello.html")
    } else {
        (format!("HTTP/1.1 404 NOT FOUND{crlf}"), "404.html")
    };

    let content = fs::read_to_string(html).unwrap();
    let content_length = content.len();
    let header = format!("Content-Length: {content_length}{crlf}{crlf}");
    let response = format!("{status_line}{header}{content}");

    stream.write_all(response.as_bytes()).unwrap();


}
