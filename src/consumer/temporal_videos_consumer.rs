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
use crate::node;
use crate::consumer;
use consumer::node_creation_error::NodeCreationError;
use crate::consumer::temporal_video_message::TemporalVideoMessage;

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
            let fetched_node = node::api::get_node(node_id).await?;
            self.node.insert(node_id, fetched_node);
        }

        let node = self.node.get_mut(&node_id).unwrap();
        let _ = node.connect().await?;

        Ok(node)
    }

    async fn handle_new_video(
        &mut self,
        temporal_video_message: TemporalVideoMessage
    ) -> Result<(), MessageHandlingError> {
        let node = self.get_node(temporal_video_message.node_id).await?;
        let video_bytes: Vec<u8> = node.get_video(&temporal_video_message.path).await?;
        self.save_video(
            video_bytes,
            temporal_video_message.camera_id,
            &temporal_video_message.video_date,
            &temporal_video_message.video_time
        ).await?;

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

        let temporal_video_message: serde_json::Result<TemporalVideoMessage> = serde_json::from_str(message);
        let mut has_errors = true;

        if temporal_video_message.is_ok() {
            let handling_result = self.handle_new_video(temporal_video_message.unwrap()).await;

            if handling_result.is_ok() {
                has_errors = false;
                let args = BasicAckArguments::new(deliver.delivery_tag(), false);
                channel.basic_ack(args).await.unwrap();
            }
        }

        if has_errors {
            let args = BasicRejectArguments::new(deliver.delivery_tag(), true);
            channel.basic_reject(args).await.unwrap();
        }
    }
}
