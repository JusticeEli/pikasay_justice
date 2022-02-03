use std::{io, thread};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;
use colour::*;

const SERVER_ADDRESS: &str = "127.0.0.1:6000";
const SLEEP_DURATION_BEFORE_MESSAGE_READ_RETRY_OCCURS: Duration = Duration::from_millis(100);
//const MAXIMUM_BYTES_TO_BE_READ_FROM_STREAM: u32 = 1024;

fn main() {
    init_client()
}

fn init_client() {
    blue_ln!("Program started");

    let mut tcp_stream = setup_tcp_connection();
    fetch_messages_from_remote(tcp_stream.try_clone().unwrap());
    magenta_ln!("Write message");
    loop {
        let mut message = String::new();
        dark_cyan!("You >>>");
        io::stdin().read_line(&mut message).expect("could not read message from std in");
        send_message_to_remote(tcp_stream.try_clone().unwrap(), message.trim())
    }
}

fn setup_tcp_connection() -> TcpStream {
    loop {
        match TcpStream::connect(SERVER_ADDRESS) {
            Ok(tcp_stream) => {
                green_ln!("Connection Established");
                tcp_stream.set_nonblocking(true);
                return tcp_stream;
            }
            Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                red_ln!("Server port has not yet been openned");
                long_sleep()
            }
            Err(e) => {
                red_ln!("Error:{}",e);
                long_sleep();
            }
        }
    }
}

fn send_message_to_remote(mut tcp_stream: TcpStream, message: &str) {
    let message = message.trim();
    tcp_stream.write(message.as_bytes()).expect("could not write to stream");
}

fn fetch_messages_from_remote(mut tcp_stream: TcpStream) {
    thread::spawn(move || loop {
        let mut buf = vec![0; 1024];
        match tcp_stream.read(&mut buf) {
            Ok(size) => {
                buf.resize(size, 0);
                print_received_message(&buf);
                buf.clear();
            }
            Err(ref e)if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                //  panic!("Error trying to read from stream:{:?}", e);
                red_ln!("Please wait as we try to establish connection with the Server");
                long_sleep();
                init_client();
            }
            Err(e) => {
                red_ln!("Error: {:?}",e);
                break;
            }
        }
        sleep();
    });
}

fn print_received_message(message: &[u8]) {
    let message = String::from_utf8_lossy(message);
    blue_ln!("\n{}",message.to_string());
}

fn sleep() {
    thread::sleep(SLEEP_DURATION_BEFORE_MESSAGE_READ_RETRY_OCCURS);
}

fn long_sleep() {
    thread::sleep(Duration::from_secs(5));
}

fn log(message: &str) {
    yellow_ln!("{}",message);
}