use std::fs;
use std::io::Write;
use proto::node_client::NodeClient;
use proto::StreamVideoRequest;
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

    pub async fn connect(&mut self) {
        if self.client.is_none() {
            self.client = Some(
                NodeClient::connect(
                    self.get_connection_string()
                ).await.unwrap()
            )
        }
    }

    pub async fn turn_off(&mut self) -> Result<(), tonic::transport::Error> {
        let local_node = self.client.as_mut().unwrap();
        let request = tonic::Request::new(());

        let _ = local_node.stop(request).await;

        return Ok(())
    }

    pub async fn stream_video(&mut self, path: &str) {
        let local_node = self.client.as_mut().unwrap();

        let request = StreamVideoRequest { path: String::from(path) };
        let mut stream = local_node.stream_video(request)
            .await
            .unwrap()
            .into_inner();

        let mut video: Vec<u8> = vec![];

        while let Some(item) = stream.next().await {
            let mut unwrapped_item = item.unwrap();
            video.append(&mut unwrapped_item.data);
        }

        let file_name = path.split('/').last().unwrap();
        let mut file = fs::File::create(&file_name).unwrap();
        let _ = file.write(&video).unwrap();
    }

    fn get_connection_string(&self) -> String {
        let mut connection_string: String = String::from("http://");
        connection_string.push_str(self.host.as_str());
        connection_string.push(':');
        connection_string.push_str(&self.port.to_string());
        return connection_string
    }
}
