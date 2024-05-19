use crate::api::api;
use crate::api::api_call_error::APICallError;
use crate::camera::camera::Camera;

pub async fn get_all_cameras() -> Result<Vec<Camera>, APICallError> {
    let api_response = api::get_api("/cameras").await?;

    let cameras: Vec<Camera> = serde_json::from_str(&api_response).unwrap();

    Ok(cameras)
}
