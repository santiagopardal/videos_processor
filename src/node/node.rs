use proto::{ node_client::NodeClient, StreamVideoRequest };
use tonic::{ codegen::tokio_stream::StreamExt };
use std::time::Instant;
use serde::{Deserialize};

use crate::node::video_download_error::VideoDownloadError;

mod proto {
    tonic::include_proto!("node");
}

#[derive(Deserialize)]
pub struct Node {
    ip: String,
    port: u16,
    #[serde(skip_deserializing)]
    client: Option<NodeClient<tonic::transport::Channel>>
}

impl Node {
    pub async fn connect(&mut self) -> Result<(), tonic::transport::Error> {
        if self.client.is_none() {
            self.client = Some(
                NodeClient::connect(self.get_connection_string()).await?
            )
        }

        Ok(())
    }

    pub async fn get_video(&mut self, path: &str) -> Result<Vec<u8>, VideoDownloadError> {
        let start: Instant = Instant::now();
        let client: &mut NodeClient<tonic::transport::Channel> = self.client.as_mut().unwrap();

        let request = StreamVideoRequest { path: String::from(path) };

        let mut stream: tonic::Streaming<Vec<u8>> = client.stream_video(request)
            .await?
            .into_inner();

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
        let host = std::env::var("RABBIT_HOST").unwrap();
        let connection_string: String =
            String::from("http://") + &host + &String::from(":50051");
        return connection_string
    }
}
