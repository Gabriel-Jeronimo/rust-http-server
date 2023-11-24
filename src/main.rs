use std::net::{TcpListener, TcpStream};

fn handle(stream: TcpStream) {
    // TODO: read stream data
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle(stream?);
    }

    Ok(())
}
