use std::{
    fs,
    io::Read,
    io::{Result, Write},
    net::{TcpListener, TcpStream},
    thread,
};

struct Request {
    httpVerb: String,
    route: String,
}

fn handle(mut stream: TcpStream) -> Result<()> {
    let data = &mut [0; 128];
    stream.read(data)?;

    let request = http_request_to_object(&String::from_utf8_lossy(data))?;

    let content = fs::read_to_string(format!("www/{}", request.route));

    let response: String;

    match content {
        Ok(v) => response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", v),
        Err(e) => response = "HTTP/1.1 404 Not Found\r\n".to_string(),
    }

    let _ = stream.write(response.as_bytes());

    Ok(())
}

fn main() -> Result<()> {
    const PORT: &str = "8080";

    let listener = TcpListener::bind(format!("{}:{}", "localhost", PORT))?;

    for stream in listener.incoming() {
        thread::spawn(|| {
            handle(stream.unwrap());
        });
    }

    Ok(())
}

fn http_request_to_object(request: &str) -> Result<Request> {
    let splited: Vec<&str> = request.split(" ").collect();
    let mut route = splited[1];

    if route == "/" {
        route = "index.html";
    }

    Ok(Request {
        httpVerb: splited[0].to_string(),
        route: route.replace("/", "").to_string(),
    })
}
