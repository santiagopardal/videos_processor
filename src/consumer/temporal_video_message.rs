use crate::json_utils::json_field_missing_error::JSONFieldMissingError;
use crate::json_utils::json_utils;

const REQUIRED_PROPERTIES: [&str; 5] = ["node", "camera", "path", "date", "time"];

pub struct TemporalVideoMessage {
    pub node_id: u32,
    pub camera_id: u32,
    pub path: String,
    pub video_date: String,
    pub video_time: String
}

impl TemporalVideoMessage {
    pub fn from_json(temporal_video: &serde_json::Value) -> Result<Self, JSONFieldMissingError> {
        json_utils::validate_keys_in_json(&temporal_video, Vec::from(REQUIRED_PROPERTIES))?;

        let node_id = temporal_video["node"].as_i64().unwrap() as u32;
        let camera_id = temporal_video["camera"].as_i64().unwrap() as u32;
        let path = String::from(temporal_video["path"].as_str().unwrap());

        let video_date = String::from(temporal_video["date"].as_str().unwrap());
        let video_time = temporal_video["time"].as_str().unwrap().replace(":", "-");

        Ok(TemporalVideoMessage { node_id, camera_id, path, video_date, video_time })
    }
}
