extern crate buffer;
extern crate regex;


use regex::Regex;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let l = TcpListener::bind("localhost:3000").unwrap();

    for s in l.incoming() {
      let s = s.unwrap();
      h(s);
    }
}

fn h(mut s: TcpStream) {
  let mut peek_buffer = vec![0; 512];

  let request_size = s.peek(&mut peek_buffer).unwrap();

  let request = String::from_utf8_lossy(&peek_buffer[..]);
  let content_length = get_content_length(&request);
  let buffer_size = if content_length > request_size {
      content_length + 512
    } else {
      request_size
  };

  let mut buffer = vec![0; buffer_size];
  s.read(&mut buffer).unwrap();

  let full_request = String::from_utf8_lossy(&buffer[..]);

  println!("{}", full_request);
  println!("Data: {}", request_data(&full_request));

  s.write("Hello world".as_bytes()).unwrap();
  s.flush().unwrap();
}

fn get_content_length(request: &str) -> usize{
  let regex = Regex::new("Content-Length:.+").unwrap();
  let target = regex.find(request).unwrap();
  let length_header = &target.as_str();
  let length_header = &length_header[length_header.len()-4..length_header.len()];
  let number = str::parse(length_header.trim()).unwrap();
  number
}

fn request_data(request: &str) -> &str {
  let chunks = request.split("\r\n");
  let data = chunks.last();
  data.unwrap()
}
