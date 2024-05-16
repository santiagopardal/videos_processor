use reqwest;
use serde_json;
use std::env;
use api::api_call_error::APICallError;
use crate::api;


pub async fn call_api(endpoint: &str) -> Result<serde_json::Value, APICallError> {
    let base_path_unwrapped: String = env::var("API_URL").expect("API_URL must be set");
    let base_path = base_path_unwrapped.as_str();

    let url: String = String::new() + base_path + endpoint;

    let response = reqwest::get(url).await?.text().await?;

    return Ok(serde_json::from_str(&response)?);
}
