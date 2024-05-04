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

    pub async fn get_video(&mut self, path: &str) -> Vec<u8> {
        let request = StreamVideoRequest { path: String::from(path) };

        let mut stream = self.client.as_mut().unwrap()
            .stream_video(request)
            .await.unwrap()
            .into_inner();

        let mut video: Vec<u8> = vec![];

        while let Some(response) = stream.next().await {
            let mut unwrapped_response  = response.unwrap();
            video.append(&mut unwrapped_response.data);
        }

        return video;
    }

    fn get_connection_string(&self) -> String {
        let mut connection_string: String = String::from("http://");
        connection_string.push_str(self.host.as_str());
        connection_string.push(':');
        connection_string.push_str(&self.port.to_string());
        return connection_string
    }
}
