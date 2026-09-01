use async_nats::jetstream::{self, stream::StorageType};
use swarm::{config::AppConfig, consumer::NatsConsumer};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let config = AppConfig::load().expect("Could not load configuration.");

    let nats_client = async_nats::connect(config.nats.server_ip.clone()).await?;
    let nats_js = jetstream::new(nats_client);
    let nats_stream = nats_js.get_or_create_stream(jetstream::stream::Config {
        name: config.nats.stream_name.clone(),
        subjects: vec![config.nats.root_subject.clone()],
        storage: StorageType::Memory,
        ..Default::default()
    }).await?;

    let consumer = NatsConsumer::new(config.nats, nats_stream).await?;
    let consumer_thread = tokio::spawn(async move {
        println!("Beekeeper working");
        consumer.run().await;
    });
    consumer_thread.await?;

    Ok(())
}