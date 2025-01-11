use std::env;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::thread;
use std::time::Duration;

fn scan_port(host: &str, port: u16, timeout: u64) -> bool {
    let ip: IpAddr = host.parse().expect("Invalid IP address");
    let addr = SocketAddr::new(ip, port);

    if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(timeout)) {
        println!("Port {} is open", port);
        true
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <IP_ADDRESS>", args[0]);
        return;
    }

    let host = &args[1];
    let start_port = 1;
    let end_port = 1024;
    let timeout = 1;

    println!("Scanning ports on {}...", host);

    let mut handles = vec![];

    for port in start_port..=end_port {
        let host = host.to_string();
        let handle = thread::spawn(move || {
            scan_port(&host, port, timeout);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Scan completed.");
}
