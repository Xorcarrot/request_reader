use std::net::TcpListener;
use std::io::{stdin, Read, Write};
use std::str::from_utf8;
use std::env;

const REQUEST_SIZE: usize = 4096 * 8;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = match args.get(1) {
        Some(arg) => {
            let argument: String;
            if arg.len() == 0 { argument = collect_argument() }
            else { argument = arg.clone() }
            argument
        }
        None => { collect_argument() }
    };

    let port: u32 = arg.trim().parse().expect("No number provided");
    let url = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(url).expect("Error initializing listener");

    println!("waiting for request on port {port}...");
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut message: [u8; REQUEST_SIZE] = [" ".as_bytes()[0]; REQUEST_SIZE];
            stream.read(&mut message).expect("can't read body");

            let address = stream.peer_addr().expect("no socket address found");
            let body = from_utf8(&message).unwrap_or("<invalid UTF-8>").trim();


            println!("{address}:\n{body}");

            let response = format!(
                "HTTP/1.1 200 OK\r\n\
                     Access-Control-Allow-Origin: *\r\n\
                     Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH\r\n\
                     Content-Length: {}\r\n\r\n\
                     {}",
                body.len(),
                body
            );            stream.write_all(response.as_bytes()).expect("can't send response")
        } else {
            println!("can't read request")
        }
    }
}

fn collect_argument() -> String {
    println!("Port: ");
    let mut arg = String::new();
    stdin().read_line(&mut arg).expect("No args given");
    arg
}
