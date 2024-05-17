use crate::api::api;
use crate::api::api_call_error::APICallError;
use crate::camera::camera::Camera;

pub async fn get_all_cameras() -> Result<Vec<Camera>, APICallError> {
    let api_response = api::call_api("/cameras").await?;

    let mut cameras: Vec<Camera> = vec![];

    for camera_json in api_response.as_array().unwrap() {
        let camera = Camera::from_json(camera_json).unwrap();
        cameras.push(camera);
    }

    Ok(cameras)
}
