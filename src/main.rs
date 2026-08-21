use std::{thread, time::{Duration}};
use rand::RngExt;

mod payload;
mod fbs;
mod buffer;

#[tokio::main]
async fn main() {
    loop {
        let lat: f64 = rand::rng().random_range(30.0..35.00);
        let lon: f64 = rand::rng().random_range(30.0..35.00);
        let alt: f64 = rand::rng().random_range(20.0..40.00);
        let payload = payload::create_payload(lat, lon, alt, 100); // 33 bytes
        println!("{:#?}", payload.len());

        thread::sleep(Duration::from_millis(1000 / 20)); // 20Hz
    }
}
