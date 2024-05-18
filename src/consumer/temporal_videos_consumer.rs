use amqprs::channel::{BasicAckArguments, BasicRejectArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use std::{io, str};
use std::path::PathBuf;
use serde_json;
use tokio;
use tokio::io::AsyncWriteExt;
use consumer::message_handling_error::MessageHandlingError;
use crate::consumer;
use crate::consumer::temporal_video_message::TemporalVideoMessage;
use crate::node::node_pool::NodePool;

pub struct TemporalVideosConsumer {
    node_pool: NodePool
}

impl TemporalVideosConsumer {
    pub fn new(node_pool: NodePool) -> Self {
        return Self { node_pool };
    }

    async fn handle_new_video(
        &mut self,
        temporal_video_message: TemporalVideoMessage
    ) -> Result<(), MessageHandlingError> {
        let node = self.node_pool.get_node(&temporal_video_message.node_id).await?;
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
            println!("Error with {}", message);
            let args = BasicRejectArguments::new(deliver.delivery_tag(), true);
            channel.basic_reject(args).await.unwrap();
        }
    }
}
