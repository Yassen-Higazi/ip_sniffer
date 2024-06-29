use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about = "Ip Sniffer", long_about = None)]
pub struct Settings {
    #[arg(short = 't', long = "threads", default_value_t = 4)]
    pub num_of_threads: u16,

    #[arg(short = 'i', long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub ip_addr: IpAddr,
}

pub fn run_sniffer(settings: Settings) -> Vec<u16> {
    let (tx, rx) = channel();

    for i in 0..settings.num_of_threads {
        let sender = tx.clone();
        let args = settings.clone();

        std::thread::spawn(move || {
            scan(args, i, sender);
        });
    }

    let mut open_ports = vec![];

    drop(tx);

    for port in rx {
        open_ports.push(port);
    }

    open_ports.sort();

    return open_ports;
}

fn scan(args: Settings, start_port: u16, sender: Sender<u16>) {
    let mut port = start_port + 1;

    loop {
        match TcpStream::connect_timeout(&SocketAddr::new(args.ip_addr, port), Duration::from_secs(1)) {
            Ok(_) => {
                print!("+");

                std::io::stdout().flush().unwrap();
                sender.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (65535 - port) <= args.num_of_threads {
            break;
        }

        port += args.num_of_threads;
    }
}
