use clap::Parser;

use sniffer::{run_sniffer, Settings};

fn main() {
    let settings = Settings::parse();

    println!("Scanning for open ports in host: {}...", settings.ip_addr);

    let open_ports = run_sniffer(settings);

    println!(" ");

    for port in open_ports {
        println!("Port: {} in host: {} is Open", port, settings.ip_addr)
    }
}