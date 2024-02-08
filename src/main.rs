use std::net::TcpStream;

fn main() {
    let adds_ip = vec![
        "192.168.1.1",
        "192.168.31.1",
        "192.168.31.15",
        "192.168.31.23",
        "192.168.31.68",
        "192.168.31.107",
        "192.168.31.123",
        "192.168.31.178",
        "192.168.31.202",
    ];
    for address in adds_ip {
        match TcpStream::connect(format!("{}:80", address)) {
            Ok(_) => println!("{}:80 is reachable", address),
            Err(_) => println!("{}:80 isn't reachable", address),
        }
    }
}
