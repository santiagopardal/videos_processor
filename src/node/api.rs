use reqwest::StatusCode;
use crate::api::{api, api_call_error::APICallError };
use crate::node::node::Node;
use serde_json::json;

pub async fn get_node(node_id: &u32) -> Result<Node, APICallError> {
    let endpoint = String::from("/node/") + &node_id.to_string();
    let response = api::get_api(&endpoint).await?;

    let node: Node = response.json().await?;

    Ok(node)
}

pub async fn register() -> Result<Node, APICallError> {
    let body = json!(
        {
            "port": 50051,
            "type": "PROCESSOR"
        }
    );
    let response = api::post_api("/node/", body).await?;

    let node: Node = response.json().await?;

    Ok(node)
}
