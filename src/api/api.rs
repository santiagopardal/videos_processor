use reqwest;
use std::env;
use api::api_call_error::APICallError;
use crate::api;


fn create_path(endpoint: &str) -> String {
    let base_path_unwrapped: String = env::var("API_URL").expect("API_URL must be set");
    let base_path = base_path_unwrapped.as_str();

    return String::from(base_path) + endpoint;
}

pub async fn get_api(endpoint: &str) -> Result<String, APICallError> {
    let url: String = create_path(endpoint);

    Ok(reqwest::get(url).await?.text().await?)
}

pub async fn post_api(endpoint: &str, body: serde_json::Value) -> Result<String, APICallError> {
    let url: String = create_path(endpoint);

    let client = reqwest::Client::new();

    let result = client.post(url).json(&body).send().await?.text().await?;

    Ok(result)
}
