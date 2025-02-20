use std::fs::File;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

fn main() {
    let mut s = TcpStream::connect(("google.com", 80)).unwrap();
    write!(s, "GET /index.html HTTP/1.0\r\n\r\n").unwrap();
    s.flush().unwrap();
    s.shutdown(Shutdown::Write).unwrap();
    let response = std::io::read_to_string(s).unwrap();
    let mut page = File::create("google-hello.html").unwrap();
    let sep = "\r\n\r\n";
    let body_start = response.find(sep).unwrap() + sep.len();
    page.write_all(response[body_start..].as_bytes()).unwrap();
}
