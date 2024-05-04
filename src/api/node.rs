use crate::api::api::call_api;
use crate::node::node::Node;

pub async fn get_node(node_id: u32) -> Node {
    let endpoint = String::from("/node/") + &node_id.to_string();
    let response = call_api(&endpoint).await;

    return Node::new(
        response["ip"].as_str().unwrap(),
        response["port"].as_i64().unwrap() as u16
    );
}
