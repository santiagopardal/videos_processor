use proto::node_client::NodeClient;
use String;

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
        let mut node = NodeClient::connect(
            self.get_connection_string()
        ).await?;

        //let request = proto::

        //let stop_result = node.stop(request).await.unwrap();

        return Ok(())
    }

    fn get_connection_string(&self) -> String {
        let mut connection_string: String = String::from("grpc://");
        connection_string.push_str(self.host.as_str());
        connection_string.push(':');
        connection_string.push_str(&self.port.to_string());
        return connection_string
    }
}
