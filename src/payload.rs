use std::time::{SystemTime, UNIX_EPOCH};
use flatbuffers::FlatBufferBuilder;

use crate::fbs::payload_generated::sensors::{Payload, PayloadArgs};

pub fn create_payload(
    lat: f64,
    lon: f64,
    alt: f64,
    battery_perc: u8
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(1024);

    let payload_offset = Payload::create(
        &mut builder,
        &PayloadArgs {
            lat: lat,
            lon: lon,
            alt: alt,
            battery_perc: battery_perc,
            timestamp: now_seconds()
        }
    );

    builder.finish_size_prefixed(payload_offset, None);
    builder.finished_data().to_vec()
}

fn now_seconds() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    duration_since_epoch.as_secs()
}