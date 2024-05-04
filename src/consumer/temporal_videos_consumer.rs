use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use std::{str};
use std::path::PathBuf;
use serde_json;
use crate::node::node::Node;
use tokio;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use std::hash::Hash;

pub struct TemporalVideosConsumer {
    node: HashMap<u32, Node>
}

async fn save_video(data: Vec<u8>, camera_id: u32, video_date: &str, video_time: String) {
    let path: PathBuf = [".", &camera_id.to_string(), video_date].iter().collect();

    tokio::fs::create_dir_all(&path).await.unwrap();

    let mut video_file = tokio::fs::File::create(
        path.join(video_time + ".mp4")
    ).await.unwrap();

    video_file.write_all(&data).await.unwrap();
}

impl TemporalVideosConsumer {
    pub async fn new() -> Self {
        let mut node = HashMap::new();
        return Self { node };
    }

    async fn get_node(&mut self, node_id: u32) -> &mut Node {
        if !self.node.contains_key(&node_id) {
            self.node.insert(node_id, Node::new("localhost", 50051));
        }

        let node = self.node.get_mut(&node_id).unwrap();
        let _ = node.connect().await.expect("Error connecting to node");

        return node;
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

        let node_id = json_data["node"].as_i64().unwrap() as u32;
        let camera_id = json_data["camera"].as_i64().unwrap() as u32;
        let path = json_data["path"].as_str().unwrap();
        let video_date = json_data["date"].as_str().unwrap();
        let video_time = json_data["time"].as_str().unwrap().replace(":", "-");

        let mut node: &mut Node = self.get_node(node_id).await;

        let video_bytes = node.get_video(path).await;

        save_video(video_bytes, camera_id, video_date, video_time).await;

        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();
    }
}
