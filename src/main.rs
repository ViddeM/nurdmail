use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    thread::sleep,
    time::{self, Duration},
};

const CRLF: [u8; 2] = [0x0D, 0x0A];

fn main() {
    let hello = "HELO nurdmail.com";

    if hello.is_ascii() == false {
        panic!("HELLO string is not valid ascii!");
    }

    println!("Trying to connect to mx server");
    let mut stream = TcpStream::connect(SocketAddr::from(([104, 47, 73, 161], 587)))
        .expect("Failed to setup TCP connection");

    println!("Connection established, sending first line");
    let bytes = to_smtp_line(hello);
    let bytes = stream
        .write(bytes.as_slice())
        .expect("Failed to send first line");

    println!("Sent {bytes}b");

    sleep(Duration::from_millis(100));

    let mut buf = String::new();
    stream
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    println!("Read string {buf}");
}

fn to_smtp_line(line: &str) -> Vec<u8> {
    let mut bytes = line.as_bytes().to_vec();
    for char in CRLF.into_iter() {
        bytes.push(char);
    }
    bytes
}
