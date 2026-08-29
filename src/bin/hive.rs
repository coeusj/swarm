use std::time::Duration;

use async_nats::jetstream::{self, stream::StorageType};
use bytes::Bytes;
use swarm::nats_config::NatsConfig;
use tokio::{net::UdpSocket, time::timeout};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let udp_read_timeout = Duration::from_secs(10);
    let udp_socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Hive situated at: {}", udp_socket.local_addr()?);

    let nats_config = NatsConfig {
        stream_name: String::from("hive"),
        root_subject: String::from("bees.>"),
        payload_subject: String::from("bees.payload")
    };
    let nats_client = async_nats::connect("127.0.0.1:4222").await?;
    let nats_js = jetstream::new(nats_client);
    nats_js.get_or_create_stream(jetstream::stream::Config {
        name: nats_config.stream_name.clone(),
        subjects: vec![nats_config.root_subject],
        storage: StorageType::Memory,
        ..Default::default()
    }).await?;

    let udp_thread = tokio::spawn(async move {
        // Allocate a buffer on the stack.
        // 65535 bytes is the maximum theoretical payload size for IPv4 UDP datagrams.
        let mut buffer = [0u8; 65535];

        loop {
            match timeout(udp_read_timeout, udp_socket.recv_from(&mut buffer)).await {
                Ok(Ok((received_bytes, src_addr))) => {
                    let raw_payload = &buffer[..received_bytes];
                    println!("Received {received_bytes} bytes from {src_addr}: {:?}", raw_payload);

                    // Convert to Bytes (zero-copy allocation where possible) for async NATS publishing
                    let payload = Bytes::copy_from_slice(raw_payload);

                    if let Err(err) = nats_js.publish(nats_config.payload_subject.clone(), payload).await {
                        eprintln!("Failed to publish to NATS: {err}");
                    }
                }
                Ok(Err(err)) => {
                    // I/O error occurred during the read operation
                    eprintln!("Socket I/O error: {err}");
                }
                Err(_elapsed) => {
                    // The timeout duration elapsed before any packet arrived
                    println!("No data received for {udp_read_timeout:?}");
                    continue;
                }
            }
        }
    });
    udp_thread.await?;

    Ok(())
}