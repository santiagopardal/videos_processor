use crate::api::api;
use crate::api::api_call_error::APICallError;
use super::super::structs::camera;

pub async fn get_all_cameras() -> Result<Vec<camera::Camera>, APICallError> {
    let api_response = api::call_api("/cameras").await?;

    let mut cameras: Vec<camera::Camera> = vec![];

    for camera_json in api_response.as_array().unwrap() {
        let camera = camera::build_from_json(camera_json);
        cameras.push(camera);
    }

    Ok(cameras)
}
