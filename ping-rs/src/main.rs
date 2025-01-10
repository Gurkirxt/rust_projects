use std::env;
use std::net::IpAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use trust_dns_resolver::TokioAsyncResolver;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <host>", args[0]);
        return;
    }

    let host = &args[1];

    // Create an asynchronous DNS resolver
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    // Perform DNS lookup
    let response = resolver.lookup_ip(host).await.unwrap();

    for ip in response.iter() {
        println!("Pinging {} ({})", host, ip);
        if let Ok(duration) = ping(ip).await {
            println!("Response from {}: time={:?}", ip, duration);
        } else {
            println!("Request timed out");
        }
    }
}

async fn ping(ip: IpAddr) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let socket = timeout(Duration::from_secs(1), TcpStream::connect((ip, 80))).await??;
    let duration = start.elapsed();
    drop(socket);
    Ok(duration)
}
