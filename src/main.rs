use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

struct Mailbox {
    inner: VecDeque<String>,
}

impl Mailbox {
    fn new() -> Mailbox {
        Mailbox { inner: VecDeque::new() }
    }

    fn write(&mut self, message: String) {
        self.inner.push_back(message);
    }

    fn read(&mut self) -> Option<String> {
        self.inner.pop_front()
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7200").unwrap();

    let mut storage = Mailbox::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                handle(&mut s, &mut storage);
            }
            Err(e) => {
                println!("A connection failed. Error: {}", e);
            }
        }
    }
}

fn handle(stream: &mut TcpStream, storage: &mut Mailbox) {
    let message = read_message(stream);
    match message.trim() {
        "READ" => {
            handle_read(stream, storage);
        }
        _ => {
            handle_write(message, storage);
        }
    }
}

fn handle_read(stream: &mut TcpStream, storage: &mut Mailbox) {
    let data = storage.read();

    match data {
        Some(message) => write!(stream, "{}", message),
        None => write!(stream, "No message in inbox!\n")
    }.ok().expect("Write failed!");
}

fn handle_write(message: String, storage: &mut Mailbox) {
    storage.write(message);
}

fn read_message(stream: &mut TcpStream) -> String {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    let res = buffered_stream.read_line(&mut read_buffer);
    res.ok().expect("An error occured while reading!");
    read_buffer
}
