use proto::node_client::NodeClient;
use proto::StreamVideoRequest;
use crate::node::video_download_error::VideoDownloadError;
use String;
use tonic;
use tonic::codegen::tokio_stream::StreamExt;
use std::time::Instant;
use crate::json_utils::json_field_missing_error::JSONFieldMissingError;
use crate::json_utils::json_utils;

const NODE_REQUIRED_FIELDS: [&str; 2] = ["host", "port"];

mod proto {
    tonic::include_proto!("node");
}

pub struct Node {
    host: String,
    port: u16,
    client: Option<NodeClient<tonic::transport::Channel>>
}

impl Node {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, JSONFieldMissingError> {
        json_utils::validate_keys_in_json(json, Vec::from(NODE_REQUIRED_FIELDS))?;

        let node = Self {
            host: String::from(json["host"].as_str().unwrap()),
            port: json["port"].as_i64().unwrap() as u16,
            client: None
        };

        Ok(node)
    }

    pub async fn connect(&mut self) -> Result<(), tonic::transport::Error> {
        if self.client.is_none() {
            self.client = Some(
                NodeClient::connect(
                    self.get_connection_string()
                ).await?
            )
        }

        Ok(())
    }

    pub async fn get_video(&mut self, path: &str) -> Result<Vec<u8>, VideoDownloadError> {
        let start = Instant::now();
        let client = self.client.as_mut().unwrap();

        let request = StreamVideoRequest { path: String::from(path) };
        let mut stream = client.stream_video(request).await?.into_inner();

        let mut video: Vec<u8> = vec![];

        while let Some(response) = stream.next().await {
            let mut unwrapped_response = response?;
            video.append(&mut unwrapped_response);
        }

        println!("It took {:.2?} to download video a of {} bytes", start.elapsed(), video.len());

        Ok(video)
    }

    fn get_connection_string(&self) -> String {
        //String::from("grcp://[") + self.host.as_str() + "]:"  + &self.port.to_string()
        let connection_string = String::from("http://192.168.100.9:50051");
        return connection_string
    }
}
