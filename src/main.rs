use std::{
    fs,
    io::Read,
    io::{Result, Write},
    net::{TcpListener, TcpStream},
};

struct Request {
    httpVerb: String,
    route: String,
}

fn handle(mut stream: TcpStream) -> Result<()> {
    let data = &mut [0; 128];
    stream.read(data)?;

    let request = httpRequestToObject(&String::from_utf8_lossy(data))?;

    println!("{}", request.route);

    let content = fs::read_to_string(format!("www/{}", request.route));

    let response: String;

    match content {
        Ok(v) => response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", v),
        Err(e) => response = "HTTP/1.1 404 Not Found\r\n".to_string(),
    }

    stream.write(response.as_bytes());

    Ok(())
}

fn main() -> Result<()> {
    const PORT: &str = "8080";

    let listener = TcpListener::bind(format!("{}:{}", "localhost", PORT))?;

    println!("app listening on port {}", PORT);
    for stream in listener.incoming() {
        handle(stream?);
    }

    Ok(())
}

fn httpRequestToObject(request: &str) -> Result<Request> {
    let splited: Vec<&str> = request.split(" ").collect();

    Ok(Request {
        httpVerb: splited[0].to_string(),
        route: splited[1].replace("/", "").to_string(),
    })
}
