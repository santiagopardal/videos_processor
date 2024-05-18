use crate::api::api;
use crate::api::api_call_error::APICallError;
use crate::node::node::Node;

pub async fn get_node(node_id: &u32) -> Result<Node, APICallError> {
    let endpoint = String::from("/node/") + &node_id.to_string();
    let response = api::call_api(&endpoint).await?;

    let node: Node = serde_json::from_str(&response)?;

    Ok(node)
}
