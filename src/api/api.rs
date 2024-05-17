use reqwest;
use std::env;
use api::api_call_error::APICallError;
use crate::api;


pub async fn call_api(endpoint: &str) -> Result<String, APICallError> {
    let base_path_unwrapped: String = env::var("API_URL").expect("API_URL must be set");
    let base_path = base_path_unwrapped.as_str();

    let url: String = String::new() + base_path + endpoint;

    Ok(reqwest::get(url).await?.text().await?)
}
