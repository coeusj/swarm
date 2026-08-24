use std::time::{SystemTime, UNIX_EPOCH};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::fbs::payload_generated::sensors::{Payload, PayloadBuilder};

pub fn create_payload<'a>(
    fb_builder: &mut FlatBufferBuilder<'a>,
    lat: f64,
    lon: f64,
    alt: f64,
    battery_perc: u8
) -> WIPOffset<Payload<'a>> {
    let mut payload = PayloadBuilder::new(fb_builder);
    payload.add_lat(lat);
    payload.add_alt(alt);
    payload.add_lon(lon);
    payload.add_battery_perc(battery_perc);
    payload.add_timestamp(now_seconds());
    payload.finish()
}

fn now_seconds() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    duration_since_epoch.as_secs()
}