use std::{io, net::UdpSocket, thread, time::Duration};
use flatbuffers::FlatBufferBuilder;
use rand::RngExt;
use swarm::{buffer::{self, RingBuffer}, payload};

fn main() -> io::Result<()> {
    const BUFFER_SIZE: usize = 10;
    let buffer: RingBuffer<Vec<u8>, BUFFER_SIZE> = buffer::RingBuffer::new();

    let udp_sender_ip = "127.0.0.1:0"; // the port is automatically assigned by the OS
    let udp_receiver_ip  = "127.0.0.1:8080";
    let udp_socket = UdpSocket::bind(udp_sender_ip)?;
    println!("Bee flying at: {}", udp_socket.local_addr()?);

    let sender = thread::spawn(move || {
        let mut fb_builder = FlatBufferBuilder::with_capacity(1024);

        loop {
            fb_builder.reset();

            let lat: f64 = rand::rng().random_range(30.0..35.00);
            let lon: f64 = rand::rng().random_range(30.0..35.00);
            let alt: f64 = rand::rng().random_range(20.0..40.00);

            let root_offset = payload::create_payload(
                &mut fb_builder,
                lat,
                lon,
                alt,
                100); // 33 bytes
            fb_builder.finish(root_offset, None);

            // Access encoded slice without allocating new memory
            let encoded_bytes: &[u8] = fb_builder.finished_data();

            match udp_socket.send_to(encoded_bytes, udp_receiver_ip) {
                Ok(_) => {
                    println!("Payload sent directly to network ({} bytes)", encoded_bytes.len());
                }
                Err(err) => {
                    eprintln!("Failed to send UDP packet: {err:?}");
                    _ = buffer.push(encoded_bytes.to_vec()); // TODO: should check if it fails or not
                }
            }

            thread::sleep(Duration::from_millis(1000 / 20)); // 20Hz
        }
    });

    sender.join().unwrap();

    Ok(())
}
