use std::{io::ErrorKind, net::UdpSocket, time::Duration};

fn main() -> std::io::Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:8080")?;
    let read_timeout = Duration::from_secs(10);
    udp_socket.set_read_timeout(Some(read_timeout))?;
    println!("Hive situated at: {}", udp_socket.local_addr()?);

    // Allocate a buffer on the stack.
    // 65535 bytes is the maximum theoretical payload size for IPv4 UDP datagrams.
    let mut buffer = [0u8; 65535];

    loop {
        match udp_socket.recv_from(&mut buffer) {
            Ok((received_bytes, src_addr)) => {
                let payload = &buffer[..received_bytes];
                println!("Received {received_bytes} bytes from {src_addr}: {:?}", payload);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut => {
                // Timeout elapsed with no incoming packets!
                println!("No data received for {read_timeout:?}");
                continue;
            }
            Err(err) => {
                eprintln!("Error receiving UDP packet: {err}");
            }
        }
    }
}