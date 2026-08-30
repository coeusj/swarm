use std::time::Duration;

use async_nats::jetstream::{self, consumer::PullConsumer, stream::Stream};
use futures::StreamExt;

use crate::config::NatsConfig;

pub struct NatsConsumer {
    pull_consumer: PullConsumer,
}

impl NatsConsumer {
    pub async fn new(config: NatsConfig, nats_stream: Stream) -> Result<Self, anyhow::Error> {
        let consumer = nats_stream
            .get_or_create_consumer(
                &config.stream_name,
                jetstream::consumer::pull::Config {
                    durable_name: Some(config.stream_name.to_string()),
                    ack_policy: jetstream::consumer::AckPolicy::Explicit,
                    filter_subject: config.payload_subject,
                    ..Default::default()
                },
            )
            .await?;

        Ok(Self {
            pull_consumer: consumer,
        })
    }

    pub async fn run(&self) {
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("Shutdown signal received. Stopping consume loop");
                    break;
                }
                result = self.consume() => {
                    if let Err(err) = result {
                        eprintln!("Error processing batch: {err}. Retrying in 1s");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }

    async fn consume(&self) -> Result<(), anyhow::Error> {
        let mut batch = self
            .pull_consumer
            .fetch()
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
}
