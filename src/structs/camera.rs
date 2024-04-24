pub struct CameraConfigurations {
    pub recording: bool,
    pub sensitivity: f32
}

pub struct Camera {
    pub id: u32,
    pub model: String,
    pub ip: String,
    pub http_port: u16,
    pub streaming_port: u16,
    pub user: String,
    pub password: String,
    pub width: u16,
    pub height: u16,
    pub framerate: u16,
    pub name: String,
    pub node: u32,
    pub configurations: CameraConfigurations
}


pub fn build_from_json(camera_json: &serde_json::Value) -> Camera {
    let camera_configurations = CameraConfigurations {
        sensitivity: camera_json["configurations"]["sensitivity"].as_f64().unwrap() as f32,
        recording: camera_json["configurations"]["recording"].as_bool().unwrap(),
    };
    return Camera {
        id: camera_json["id"].as_i64().unwrap() as u32,
        model: String::from(camera_json["model"].as_str().unwrap()),
        ip: String::from(camera_json["ip"].as_str().unwrap()),
        http_port: camera_json["http_port"].as_i64().unwrap() as u16,
        streaming_port: camera_json["streaming_port"].as_i64().unwrap() as u16,
        user: String::from(camera_json["user"].as_str().unwrap()),
        password: String::from(camera_json["password"].as_str().unwrap()),
        width: camera_json["width"].as_i64().unwrap() as u16,
        height: camera_json["height"].as_i64().unwrap() as u16,
        framerate: camera_json["framerate"].as_i64().unwrap() as u16,
        name: String::from(camera_json["name"].as_str().unwrap()),
        node: camera_json["node"].as_i64().unwrap() as u32,
        configurations: camera_configurations
    };
}
