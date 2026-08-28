use async_nats::jetstream::{self, stream::StorageType};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    // Establish NATS connection
    let client = async_nats::connect("127.0.0.1:4222").await?;

    // Instantiate JetStream context
    let js = async_nats::jetstream::new(client);

    // Ensure the target Stream exists
    js.get_or_create_stream(jetstream::stream::Config {
        name: "TESTS".to_string(),
        subjects: vec!["test.>".into()],
        storage: StorageType::Memory,
        ..Default::default()
    }).await?;

    js.publish("test.payload", "test-payload-1".into()).await?;
    js.publish("test.payload", "test-payload-2".into()).await?;

    println!("Message published successfully in stream");
    Ok(())
}