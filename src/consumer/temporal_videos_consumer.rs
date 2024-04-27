use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use std::str;
use serde_json;
use crate::node;

pub struct TemporalVideosConsumer;

impl TemporalVideosConsumer {
    pub fn new() -> Self {
        return Self;
    }
}

#[async_trait]
impl AsyncConsumer for TemporalVideosConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let s: &str = match str::from_utf8(&content) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let json_data: serde_json::Value = serde_json::from_str(&s).unwrap();

        println!("Message is: '{}'", s);
        println!("JSON is: '{}'", json_data);

        let local_node = node::node::Node::new("192.168.100.9", 50051);
        let id = json_data["video_id"].as_i64().unwrap();
        local_node.stream_video(id as u32).await;

        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();

        println!("ACKd message!");
        println!();
    }
}
