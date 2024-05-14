use proto::node_client::NodeClient;
use proto::StreamVideoRequest;
use crate::node::errors::VideoDownloadError;
use String;
use tonic;
use tonic::codegen::tokio_stream::StreamExt;

mod proto {
    tonic::include_proto!("node");
}

pub struct Node {
    host: String,
    port: u16,
    client: Option<NodeClient<tonic::transport::Channel>>
}

impl Node {
    pub fn new(host: &str, port: u16) -> Self {
        return Self {
            host: String::from(host),
            port,
            client: None
        }
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
        let client = self.client.as_mut().unwrap();

        let request = StreamVideoRequest { path: String::from(path) };
        let mut stream = client.stream_video(request).await?.into_inner();

        let mut video: Vec<u8> = vec![];

        while let Some(response) = stream.next().await {
            let mut unwrapped_response = response?;
            video.append(&mut unwrapped_response);
        }

        Ok(video)
    }

    fn get_connection_string(&self) -> String {
        //String::from("grcp://[") + self.host.as_str() + "]:"  + &self.port.to_string()
        let connection_string = String::from("http://192.168.100.9:50051");
        return connection_string
    }
}
