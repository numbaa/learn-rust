extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    //parse?
    let addr = "127.0.0.1:7878".parse().unwrap();
    //let stream = TcpStream::connect(&addr);
    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("created stream");
        io::write_all(stream, "hello, world").then(|result| {
            println!("wrote to stream; success={:?}", result.is_ok());
            Ok(())
        })
    })
    .map_err(|err| {
        println!("connection error {:?}", err);
    });
    println!("About to start tokio...");
    tokio::run(client);
    println!("Tokio stoped");
}
