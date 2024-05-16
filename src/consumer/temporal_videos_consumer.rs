use amqprs::channel::{BasicAckArguments, BasicRejectArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use std::{io, str};
use std::path::PathBuf;
use serde_json;
use crate::node::node::Node;
use tokio;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use consumer::message_handling_error::MessageHandlingError;
use api::node;
use crate::{api, consumer};
use consumer::node_creation_error::NodeCreationError;

pub struct TemporalVideosConsumer {
    node: HashMap<u32, Node>
}

impl TemporalVideosConsumer {
    pub async fn new() -> Self {
        let node = HashMap::new();
        return Self { node };
    }

    async fn get_node(&mut self, node_id: u32) -> Result<&mut Node, NodeCreationError> {
        if !self.node.contains_key(&node_id) {
            let fetched_node = node::get_node(node_id).await?;
            self.node.insert(node_id, fetched_node);
        }

        let node = self.node.get_mut(&node_id).unwrap();
        let _ = node.connect().await?;

        Ok(node)
    }

    async fn handle_new_video(
        &mut self,
        node_id: u32,
        camera_id: u32,
        path: &str,
        date: &str,
        time: &str
    ) -> Result<(), MessageHandlingError> {
        let node = self.get_node(node_id).await?;
        let video_bytes: Vec<u8> = node.get_video(path).await?;
        self.save_video(video_bytes, camera_id, date, time).await?;

        Ok(())
    }

    async fn save_video(
        &mut self,
        data: Vec<u8>,
        camera_id: u32,
        video_date: &str,
        video_time: &str
    ) -> Result<(), io::Error> {
        let path: PathBuf = [".", &camera_id.to_string(), video_date].iter().collect();
        tokio::fs::create_dir_all(&path).await?;

        let video_path = path.join(String::from(video_time) + ".mp4");

        let mut video_file = tokio::fs::File::create(video_path).await?;
        video_file.write_all(&data).await?;

        Ok(())
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
        let message: &str = str::from_utf8(&content).unwrap();
        let json_data: serde_json::Value = serde_json::from_str(&message).unwrap();

        let node_id = json_data["node"].as_i64().unwrap() as u32;
        let camera_id = json_data["camera"].as_i64().unwrap() as u32;
        let path = json_data["path"].as_str().unwrap();
        let video_date = json_data["date"].as_str().unwrap();
        let video_time = json_data["time"].as_str().unwrap().replace(":", "-");

        let handling_result = self.handle_new_video(node_id, camera_id, path, video_date, &video_time).await;

        if handling_result.is_ok() {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        } else {
            let args = BasicRejectArguments::new(deliver.delivery_tag(), true);
            channel.basic_reject(args).await.unwrap();
        }
    }
}
