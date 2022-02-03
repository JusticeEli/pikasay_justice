use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use colour::*;
const DEFAULT_THREADS: u16 = 1000;
const MAX_PORT: u16 = 500;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argument = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("-h") {
            std::process::exit(0);
        } else {
            e_red_ln!("Error:{}",err);
            std::process::exit(1);
        }
    });
    let (sender, receiver) = std::sync::mpsc::channel::<u16>();

    start_port_scanning(&argument, sender);
    let mut open_ports = Vec::new();
    for open_port in receiver {
        open_ports.push(open_port)
    }
    open_ports.sort();
    println!();
    for open_port in open_ports {
        magenta_ln!("{} is open",open_port);
    }
}

fn start_port_scanning(argument: &Arguments, sender: Sender<u16>) {

    for id in 1..=argument.threads {
        start_thread(argument,id, sender.clone());
    }
}

fn start_thread(argument: &Arguments,id: u16, sender:Sender<u16>) {
   let ipaddr=argument.ipaddr;
   let threads=argument.threads;
    std::thread::spawn(move || {
     //   println!("Thread {} started",id);
        let mut port = id;
        loop {
            if port > MAX_PORT {
                break;
            }
            if TcpStream::connect((ipaddr.clone(), port)).is_ok() {
                dark_yellow!(".");
                std::io::stdout().flush().unwrap();
                sender.send(port).unwrap();
            }
            port += threads;
        }
    });
}

struct Arguments {
    flag: String,
    threads: u16,
    ipaddr: IpAddr,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            Err("Less arguments passed")
        } else if args.len() > 4 {
            Err("To many arguments passed")
        } else {
            if let Ok(ipaddr) = IpAddr::from_str(&args[1]) {
                Ok(Arguments {
                    flag: "".to_string(),
                    threads: DEFAULT_THREADS,
                    ipaddr,
                })
            } else {
                if args[1].contains("-h") || args[1].contains("--help") {
                    blue_ln!("\
                    Usage: -h or --help :Manual for this program
                    -t or --thread :Number of thread to use in our program");
                    Err("-h")
                } else {
                    let flag = args[1].clone();
                    let threads = args[2].parse::<u16>().unwrap();
                    let ipaddr = args[3].parse::<IpAddr>().unwrap();
                    Ok(Arguments {
                        flag,
                        threads,
                        ipaddr,
                    })
                }
            }
        }
    }
}