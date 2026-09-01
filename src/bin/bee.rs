use std::{net::UdpSocket, thread, time::Duration};
use flatbuffers::FlatBufferBuilder;
use rand::RngExt;
use swarm::{buffer::{self, RingBuffer}, config::AppConfig, payload};

fn main() -> Result<(), anyhow::Error> {
    let bee_id = swarm::get_bee_id_from_args()?;
    let config = AppConfig::load().expect("Could not load configuration.");

    const BUFFER_SIZE: usize = 10;
    let buffer: RingBuffer<Vec<u8>, BUFFER_SIZE> = buffer::RingBuffer::new();

    let udp_socket = UdpSocket::bind(config.bee.udp_ip.clone())?;
    println!("Bee flying at: {}", udp_socket.local_addr()?);

    let mut sequence_number: u64 = 0;
    let sender = thread::spawn(move || {
        let mut fb_builder = FlatBufferBuilder::with_capacity(1024);

        loop {
            fb_builder.reset();

            let lat: f64 = rand::rng().random_range(30.0..35.00);
            let lon: f64 = rand::rng().random_range(30.0..35.00);
            let alt: f64 = rand::rng().random_range(20.0..40.00);

            let root_offset = payload::create_payload(
                &mut fb_builder,
                bee_id,
                lat,
                lon,
                alt,
                100,
                sequence_number);
            fb_builder.finish(root_offset, None);

            // Access encoded slice without allocating new memory
            let encoded_bytes: &[u8] = fb_builder.finished_data();

            match udp_socket.send_to(encoded_bytes, config.hive.udp_ip.clone()) {
                Ok(_) => {
                    println!("Payload sent directly to network ({} bytes)", encoded_bytes.len());
                }
                Err(err) => {
                    eprintln!("Failed to send UDP packet: {err:?}");
                    let _ = buffer.push(encoded_bytes.to_vec()); // TODO: should check if it fails or not
                }
            }

            sequence_number += 1;
            thread::sleep(Duration::from_millis(1000 / config.bee.update_frequency_hz));
        }
    });

    sender.join().unwrap();

    Ok(())
}
