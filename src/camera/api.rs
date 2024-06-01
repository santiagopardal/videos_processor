use crate::api::api;
use crate::api::api_call_error::APICallError;
use crate::camera::camera::Camera;

pub async fn get_all_cameras_in_node(node_id: &u32) -> Result<Vec<Camera>, APICallError> {
    let endpoint = String::from("/node/") + &node_id.to_string() + "/cameras";
    let response = api::get_api(&endpoint).await?;

    let cameras: Vec<Camera> = response.json().await?;

    Ok(cameras)
}
