use async_nats::jetstream::{self, stream::StorageType};
use swarm::{consumer_worker::ConsumerWorker, nats_config::NatsConfig};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let nats_config = NatsConfig {
        stream_name: String::from("hive"),
        root_subject: String::from("bees.>"),
        payload_subject: String::from("bees.payload")
    };

    let nats_client = async_nats::connect("127.0.0.1:4222").await?;
    let nats_js = jetstream::new(nats_client);
    let nats_stream = nats_js.get_or_create_stream(jetstream::stream::Config {
        name: nats_config.stream_name.clone(),
        subjects: vec![nats_config.root_subject.clone()],
        storage: StorageType::Memory,
        ..Default::default()
    }).await?;

    let consumer = ConsumerWorker::new(nats_config, nats_stream).await?;
    let consumer_thread = tokio::spawn(async move {
        println!("Starting consumer");
        consumer.run().await;
    });
    consumer_thread.await?;

    Ok(())
}