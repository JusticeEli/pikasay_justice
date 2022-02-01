use std::io::{prelude, Read, Write};

use std::net::{TcpListener, TcpStream};

use WebServer::thread_pool;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = thread_pool::ThreadPool::new(5);
    for stream in tcp_listener.incoming() {
        let tcp_stream = stream.unwrap();
        thread_pool.execute(move || {
            handle_connection(tcp_stream);
        });
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let mut buf = [0; 1024];
    tcp_stream.read(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf);

    let content = std::fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK \r\nContent-Length: {}\r\n\r\n{}", content.len(), content);

   // let response = format!("HTTP/1.1 200 OK\r\n");
    tcp_stream.write(response.as_bytes()).unwrap();
    tcp_stream.flush().unwrap()
}

