use reqwest::Client;
use crate::api;

pub async fn generate_text(client: &Client, base_url: &str, prompt: &str, max_length: u32) -> Result<String, Box<dyn std::error::Error>> {
    api::generate_text(client, base_url, prompt, max_length).await
}
