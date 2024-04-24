use reqwest;
use serde_json;
use std::env;


pub async fn call_api(endpoint: &str) -> serde_json::Value {
    let base_path_unwrapped: String = env::var("API_URL").unwrap();
    let base_path = base_path_unwrapped.as_str();
    let mut url: String = String::new();
    url.push_str(base_path);
    url.push_str(endpoint);

    let response = reqwest::get(url).await;

    let response_unwrapped = response.unwrap();
    let response_text = response_unwrapped.text();
    let response_text_unwrapped=  response_text.await.unwrap();

    return serde_json::from_str(&response_text_unwrapped).unwrap();
}
