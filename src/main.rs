use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::fs::File;

fn main() {
    let mut s = TcpStream::connect(("google.com", 80)).unwrap();
    write!(s, "GET /index.html HTTP/1.0\r\n\r\n").unwrap();
    s.flush().unwrap();
    s.shutdown(Shutdown::Write).unwrap();
    let response = std::io::read_to_string(s).unwrap();
    let mut page = File::create("google-hello.html").unwrap();
    let body_start = response.find("\r\n\r\n").unwrap() + 4;
    page.write_all(&response[body_start..].as_bytes()).unwrap();
}
