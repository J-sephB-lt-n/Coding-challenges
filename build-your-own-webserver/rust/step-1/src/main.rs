use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
}; 
use regex::Regex;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:5000").unwrap();

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
    let key_params_regex = Regex::new(r"^([A-Z]+)\s\/(\w*)\s(.+)$").unwrap();
    let request_captures = key_params_regex.captures(first_line_http_request).unwrap(); 
    let request_method = request_captures.get(1).unwrap().as_str();
    let request_path = request_captures.get(2).unwrap().as_str();
    let request_protocol = request_captures.get(3).unwrap().as_str();

    // println!("Request: {http_request:#?}");
    
    let response_status_line = "HTTP/1.1 200 OK";
    let response_contents = format!("Requested path: {request_path}");
    let response_len = response_contents.len();

    let http_response = format!("{response_status_line}\r\nContent-Length: {response_len}\r\n\r\n{response_contents}");

    stream.write_all(http_response.as_bytes()).unwrap();
}
