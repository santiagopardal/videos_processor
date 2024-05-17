use crate::json_utils::json_field_missing_error::JSONFieldMissingError;
use crate::json_utils::json_utils;

const CAMERA_REQUIRED_FIELDS: [&str; 13] = [
    "configurations",
    "streaming_port",
    "id",
    "model",
    "ip",
    "http_port",
    "user",
    "password",
    "width",
    "height",
    "framerate",
    "name",
    "nodeId"
];

pub struct CameraConfigurations {
    pub recording: bool,
    pub sensitivity: f32
}

pub struct Camera {
    pub id: u32,
    pub model: String,
    pub ip: String,
    pub http_port: u16,
    pub streaming_port: Option<u16>,
    pub user: String,
    pub password: String,
    pub width: u16,
    pub height: u16,
    pub framerate: u16,
    pub name: String,
    pub node: u32,
    pub configurations: CameraConfigurations
}


impl Camera {
    pub fn from_json(camera_json: &serde_json::Value) -> Result<Self, JSONFieldMissingError> {
        json_utils::validate_keys_in_json(&camera_json, Vec::from(CAMERA_REQUIRED_FIELDS))?;

        let camera_configurations = CameraConfigurations {
            sensitivity: camera_json["configurations"]["sensitivity"].as_f64().unwrap() as f32,
            recording: camera_json["configurations"]["recording"].as_bool().unwrap(),
        };

        let streaming_port: Option<u16> = match camera_json["streaming_port"].as_i64() {
            Some(streaming_port) => Some(streaming_port as u16),
            None => None
        };

        let camera = Camera {
            id: camera_json["id"].as_i64().unwrap() as u32,
            model: String::from(camera_json["model"].as_str().unwrap()),
            ip: String::from(camera_json["ip"].as_str().unwrap()),
            http_port: camera_json["http_port"].as_i64().unwrap() as u16,
            streaming_port,
            user: String::from(camera_json["user"].as_str().unwrap()),
            password: String::from(camera_json["password"].as_str().unwrap()),
            width: camera_json["width"].as_i64().unwrap() as u16,
            height: camera_json["height"].as_i64().unwrap() as u16,
            framerate: camera_json["framerate"].as_i64().unwrap() as u16,
            name: String::from(camera_json["name"].as_str().unwrap()),
            node: camera_json["nodeId"].as_i64().unwrap() as u32,
            configurations: camera_configurations
        };

        Ok(camera)
    }
}
