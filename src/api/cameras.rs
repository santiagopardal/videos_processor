use crate::api::api::call_api;
use super::super::structs::camera;

pub async fn get_all_cameras() -> Vec<camera::Camera> {
    let api_response = call_api("/cameras").await;

    let mut cameras: Vec<camera::Camera> = Vec::new();

    for camera_json in api_response.as_array().unwrap() {
        let camera = camera::build_from_json(camera_json);
        cameras.push(camera);
    }

    return cameras;
}
