use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NatsConfig {
    pub server_ip: String,
    pub stream_name: String,
    pub root_subject: String,
    pub payload_subject: String
}

#[derive(Deserialize, Clone)]
pub struct HiveConfig {
    pub udp_ip: String,
    pub udp_read_timeout_seconds: u64
}

#[derive(Deserialize, Clone)]
pub struct BeeConfig {
    pub udp_ip: String,
    pub update_frequency_hz: u64
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub nats: NatsConfig,
    pub hive: HiveConfig,
    pub bee: BeeConfig
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("Config").required(false));

        let config = builder.build()?;
        config.try_deserialize()
    }
}