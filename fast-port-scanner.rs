// Fast Port Scanner - Python to Rust conversion
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn tcp_connect(ip: &str, port: u16, timeout: Duration, output: Arc<Mutex<HashMap<u16, String>>>) {
    let socket_addr = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = match socket_addr.parse() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    let stream = TcpStream::connect_timeout(&socket_addr, timeout);
    let mut output = output.lock().unwrap();
    if stream.is_ok() {
        output.insert(port, "Listening".to_string());
    }
}

fn scan_ports(host_ip: &str, delay: u64) {
    let timeout = Duration::from_secs(delay);
    let output = Arc::new(Mutex::new(HashMap::new()));
    let mut threads = vec![];

    for port in 0..10000 {
        let ip = host_ip.to_string();
        let output_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            tcp_connect(&ip, port, timeout, output_clone);
        });
        threads.push(handle);
    }

    for handle in threads {
        let _ = handle.join();
    }

    let output = output.lock().unwrap();
    for port in 0..10000 {
        if let Some(status) = output.get(&port) {
            println!("{}: {}", port, status);
        }
    }
}

fn main() {
    println!("Enter host IP: ");
    let mut host_ip = String::new();
    io::stdin().read_line(&mut host_ip).unwrap();
    let host_ip = host_ip.trim();

    println!("How many seconds the socket is going to wait until timeout: ");
    let mut delay = String::new();
    io::stdin().read_line(&mut delay).unwrap();
    let delay: u64 = delay.trim().parse().unwrap_or(1);

    scan_ports(&host_ip, delay);
}
