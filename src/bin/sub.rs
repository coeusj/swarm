use std::time::Duration;

use async_nats::jetstream::{self, consumer::PullConsumer, stream::StorageType};
use futures::StreamExt;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let client = async_nats::connect("localhost:4222").await?;
    let js = async_nats::jetstream::new(client);

    // Create a stream that captures any subject under `test.`
    let stream = js.get_or_create_stream(jetstream::stream::Config {
        name: "TESTS".to_string(),
        subjects: vec!["test.>".into()],
        storage: StorageType::Memory,
        ..Default::default()
    }).await?;

    // Create a durable pull consumer that delivers from the beginning
    let consumer = stream.get_or_create_consumer(
        "test-processor",
        jetstream::consumer::pull::Config {
            durable_name: Some("test-processor".to_string()),
            ack_policy: jetstream::consumer::AckPolicy::Explicit,
            filter_subject: "test.payload".to_string(),
            ..Default::default()
        }
    ).await?;

    println!("Starting consume loop..");
    loop {
        let t_consumer = consumer.clone();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("Shutdown signal received. Stopping worker loop...");
                break;
            }
            result = consume(t_consumer) => {
                if let Err(err) = result {
                    eprintln!("Error processing batch: {err}. Retrying in 1 second...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    Ok(())
}

pub async fn consume(consumer: PullConsumer) -> Result<(), anyhow::Error>{
    let mut batch = consumer.fetch()
        .max_messages(2)
        .expires(Duration::from_secs(1))
        .messages()
        .await?;

    while let Some(message_res) = batch.next().await {
        match message_res {
            Ok(message) => {
                println!("Received message {:?}", message);

                match message.ack().await {
                    Ok(_) => {
                        println!("ACK sent successfully");
                    }
                    Err(err) => {
                        eprintln!("ACK err: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Stream error: {}", err);
            }
        }
    }

    Ok(())
}