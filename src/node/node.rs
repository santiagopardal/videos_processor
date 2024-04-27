use proto::node_client::NodeClient;
use String;
use tonic;

mod proto {
    tonic::include_proto!("node");
}

pub struct Node {
    host: String,
    port: u16
}

impl Node {
    pub fn new(host: &str, port: u16) -> Self {
        return Self {
            host: String::from(host),
            port
        }
    }

    pub async fn turn_off(&self) -> Result<(), tonic::transport::Error> {
        let mut local_node = NodeClient::connect(
            self.get_connection_string()
        ).await?;

        let request = tonic::Request::new(());

        local_node.stop(request).await;

        return Ok(())
    }

    pub async fn stream_video(&self, video_id: u32) -> Result<(), tonic::transport::Error> {
        let mut local_node = NodeClient::connect(
            self.get_connection_string()
        ).await?;

        let request = tonic::Request::new(
            proto::StreamVideoRequest { video_id: video_id }
        );

        local_node.stream_video(request).await;

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
