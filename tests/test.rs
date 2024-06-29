use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, TcpListener};

use sniffer::run_sniffer;
use sniffer::Settings;

#[test]
fn it_should_detect_open_ports() {
    let result = TcpListener::bind("127.0.0.1:8080");

    match result {
        Ok(_) => {}
        Err(error) => {
            match error.kind() {
                ErrorKind::AddrInUse => {}
                _ => {
                    panic!("Could not open port 8080, err: {:?}", error)
                }
            }
        }
    }

    let settings = Settings {
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
        num_of_threads: 4,
    };

    let open_ports: Vec<u16> = run_sniffer(settings);

    assert_eq!(open_ports.contains(&(8080 as u16)), true);
}