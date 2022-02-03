use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use colour::*;
use std::sync::{Arc, mpsc::Sender, mpsc::Receiver, mpsc, Mutex};
use std::time::Duration;

const SERVER_ADDRESS: &str = "127.0.0.1:6000";
const SLEEP_DURATION_BEFORE_MESSAGE_READ_RETRY_OCCURS: Duration = Duration::from_millis(100);
//const MAXIMUM_BYTES_TO_BE_READ_FROM_STREAM: u32 = 1024;

fn sleep() {
    thread::sleep(SLEEP_DURATION_BEFORE_MESSAGE_READ_RETRY_OCCURS);
}

fn main() {
    let tcp_listener = TcpListener::bind(SERVER_ADDRESS).expect("TcpListener Unable to bind to server");
    blue_ln!("Server started");
    tcp_listener.set_nonblocking(true);
    let (sender, receiver) = mpsc::channel();

    listen_for_connections(tcp_listener, sender);
    sent_message_to_every_client(receiver);
}

fn sent_message_to_every_client(receiver: Receiver<Info>) {
    let mut clients = Vec::new();
    for mut data in receiver {
        match data {
            Info::Joined(tcp_stream) => {
                blue_ln!("{} Joined",tcp_stream.peer_addr().unwrap());
                clients.push(tcp_stream);


                let filter_list=clients.iter_mut().map(|tcp|{
                    tcp.peer_addr().unwrap().ip()
                });

               // log(&format!("filtered_list:{:?}",filter_list));

            }
            Info::Message(tcp_stream, message) => {
                let filtered_list = clients.iter_mut()
                    .filter(|tcp| {
                        tcp.peer_addr().unwrap().ne(&tcp_stream.peer_addr().unwrap())
                    });
                //log(&format!("filtered_list:{:?}",filtered_list));
                for client in filtered_list {
                    //   log(&format!("sending message to:{}", client.peer_addr().unwrap().to_string()));
                    client.write(message.as_bytes()).expect("Could not write to TcpStream");
                }
            }
            Info::Left(tcp_stream) => {
                red_ln!("{} Left",tcp_stream.peer_addr().unwrap());
            }
        }
    }
}

fn listen_for_connections(tcp_listener: TcpListener, sender: Sender<Info>) {
    let sender = Arc::new(Mutex::new(sender));
    let tcp_listener = tcp_listener.try_clone().unwrap();
    thread::spawn(move || loop {
        match tcp_listener.accept() {
            Ok(tcp_connection) => {
                let sender = Arc::clone(&sender);
                thread::spawn(move || {
                    handle_connection(tcp_connection, sender);
                });
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                panic!("Error Trying to accept connection:{:?}", e);
            }
        }

        sleep()
    });
}

enum Info {
    Joined(TcpStream)
    ,
    Message(TcpStream, String),
    Left(TcpStream),
}

fn handle_connection((mut tcp_stream, address): (TcpStream, SocketAddr), sender: Arc<Mutex<Sender<Info>>>) {
    // blue_ln!("{} :connected to server",address);

    sender.lock().unwrap().send(Info::Joined(tcp_stream.try_clone().unwrap())).expect("Could not send message to channel receiver");

    let mut buf = vec![0; 1024];
    loop {
        match tcp_stream.read(&mut buf) {
            Ok(size) => {
                buf.resize(size, 0);
                let msg = &String::from_utf8_lossy(&buf).to_string();
                magenta!("{} >>>",address);
                blue_ln!("{}",msg);
                sender.lock().unwrap().send(Info::Message(tcp_stream.try_clone().unwrap(), msg.to_string())).expect("could not send message");
                buf.clear()
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(ref e) if e.kind() == ErrorKind::ConnectionReset => {
                // red_ln!("{} has a CONNECTION_RESET ",address);
                sender.lock().unwrap().send(Info::Left(tcp_stream.try_clone().unwrap())).unwrap();
                break;
            }
            Err(ref e) if e.kind() == ErrorKind::ConnectionAborted => {
                // red_ln!("{} has a CONNECTION_ABORTED ",address);
                sender.lock().unwrap().send(Info::Left(tcp_stream.try_clone().unwrap())).unwrap();
                break;
            }
            Err(e) => {
                red_ln!("Error: {:?}",e);
                break;
            }
        }
        sleep()
    }
}

fn long_sleep() {
    thread::sleep(Duration::from_secs(5));
}

fn log(message: &str) {
    red_ln!("log:>>>>{}",message)
}