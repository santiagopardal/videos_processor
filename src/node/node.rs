use proto::node_client::NodeClient;
use String;
use tonic;

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

    pub async fn stream_video(&mut self, video_id: u32) -> Result<(), tonic::transport::Error> {
        let local_node = self.client.as_mut().unwrap();
        let request = tonic::Request::new(
            proto::StreamVideoRequest { video_id }
        );

        let _ = local_node.stream_video(request).await.unwrap();

        return Ok(())
    }

    fn get_connection_string(&self) -> String {
        let mut connection_string: String = String::from("http://");
        connection_string.push_str(self.host.as_str());
        connection_string.push(':');
        connection_string.push_str(&self.port.to_string());
        return connection_string
    }
}
