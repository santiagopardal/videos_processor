use crate::api::api;
use crate::api::api_call_error::APICallError;
use crate::camera::camera::Camera;

pub async fn get_all_cameras() -> Result<Vec<Camera>, APICallError> {
    let response = api::get_api("/cameras").await?;

    let cameras: Vec<Camera> = response.json().await?;

    Ok(cameras)
}
