use reqwest::Client;
use serde_json::json;
use std::error::Error;

// Health check function
pub async fn check_health(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/health", base_url);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Health check failed: {}", response.status()).into())
    }
}

// Generate text with AI
pub async fn generate_text(client: &Client, base_url: &str, prompt: &str, max_length: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/generate", base_url);

    let payload = json!({
        "prompt": prompt,
        "max_length": max_length
    });

    let response = client.post(&url)
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["response"].as_str().unwrap_or("No response").to_string())
}
