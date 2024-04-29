use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use std::str;
use serde_json;
use crate::node::node::Node;

pub struct TemporalVideosConsumer {
    node: Node
}

impl TemporalVideosConsumer {
    pub async fn new() -> Self {
        let mut node = Node::new("127.0.0.1", 50051);
        node.connect().await;
        return Self { node };
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

        let id = json_data["video_id"].as_i64().unwrap();
        self.node.connect().await;
        self.node.stream_video(id as u32).await.unwrap();

        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();

        println!("ACKd message!");
        println!();
    }
}
