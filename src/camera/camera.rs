use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
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
    #[serde(alias = "nodeId")]
    pub node_id: u32,
    pub recording: bool,
    pub sensitivity: f32,
}
