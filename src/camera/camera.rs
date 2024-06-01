use serde::{ Deserialize, Serialize };
use crate::node::node::Node;

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
    pub nodes: Vec<Node>,
    pub recording: bool,
    pub sensitivity: f32,
}
