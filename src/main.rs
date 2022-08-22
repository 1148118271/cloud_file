mod request;
mod html;
mod response;

use std::io::{ErrorKind, Write};
use std::thread;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use crate::request::Request;

#[tokio::main]
async fn main() {
    server_run().await;
}


async fn server_run() {
    let std_listener = std::net::TcpListener::bind("0.0.0.0:9258").unwrap();
    std_listener.set_nonblocking(true).unwrap();
    let listener = TcpListener::from_std(std_listener).unwrap();

    loop {
        let (server, addr) = listener.accept().await.unwrap();
        println!("client addr {}", addr);
        let request = Request::process(server).await;
        println!("{:?}", request);
    }
}


#[test] fn t() {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:9258").unwrap();
    let mut c = vec![];
    for i in 0..255 {
        c.push(i);
    }
    stream.write(&c).unwrap();
    thread::sleep(Duration::from_secs(60 * 60 * 12));
}