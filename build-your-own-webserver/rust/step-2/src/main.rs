use regex::Regex;
use std::{
    fs::read_to_string,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() {
    let server_url = "127.0.0.1:5000";
    let tcp_listener = TcpListener::bind(&server_url).unwrap();
    println!("Listening on http://{}", &server_url);

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // unwrap() makes crash on error
        .take_while(|line| !line.is_empty())
        .collect();

    let first_line_http_request: &str = &http_request[0];
    let key_params_regex = Regex::new(r"^([A-Z]+)\s\/(.*)\s(.+)$").unwrap();
    let request_captures = match key_params_regex.captures(first_line_http_request) {
        Some(captures) => captures,
        None => {
            panic!("Error parsing HTTP request");
        }
    };
    // let request_method = request_captures.get(1).unwrap().as_str();
    let request_path = request_captures.get(2).unwrap().as_str();
    let request_path = if request_path.is_empty() {
        "index.html"
    } else {
        request_path
    };
    let request_path = format!("www/{request_path}");
    // let request_protocol = request_captures.get(3).unwrap().as_str();

    // println!("Request: {http_request:#?}");

    let file_path = Path::new(&request_path);

    let http_response = if file_path.exists() && file_path.is_file() {
        let response_status_line = "HTTP/1.1 200 OK";
        let response_body = read_to_string(&file_path).unwrap();
        let response_len = response_body.len();
        format!("{response_status_line}\r\ncontent-length: {response_len}\r\n\r\n{response_body}")
    } else {
        let response_status_line = "HTTP/1.1 404 Not Found";
        let response_body = "<p>The requested file could not be found</p>";
        let response_len = response_body.len();
        format!("{response_status_line}\r\ncontent-length: {response_len}\r\n\r\n{response_body}")
    };

    match stream.write_all(http_response.as_bytes()) {
        Ok(()) => println!("Response sent successfully"),
        Err(e) => eprintln!("Failed to send response: {}", e),
    }
}
