use reqwest;
use std::env;
use reqwest::Response;
use api::api_call_error::APICallError;
use crate::api;


fn create_path(endpoint: &str) -> String {
    let base_path_unwrapped: String = env::var("API_URL").expect("API_URL must be set");
    let base_path = base_path_unwrapped.as_str();

    return String::from(base_path) + endpoint;
}

async fn error_on_status(response: Response) -> Result<Response, APICallError> {
    let status = response.status();

    if !status.is_success() {
        let error = APICallError::Status {
            status,
            response: response.json().await?
        };
        return Err(error);
    }

    Ok(response)
}

pub async fn get_api(endpoint: &str) -> Result<Response, APICallError> {
    let url: String = create_path(endpoint);

    let client: reqwest::Client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    Ok(error_on_status(response).await?)
}

pub async fn post_api(endpoint: &str, body: serde_json::Value) -> Result<Response, APICallError> {
    let url: String = create_path(endpoint);

    let client: reqwest::Client = reqwest::Client::new();
    let response = client.post(url).json(&body).send().await?;

    Ok(error_on_status(response).await?)
}
