use reqwest::Client;
use crate::database::{Database, AIRequest, AIResponse, GenerationStats};
use chrono::Local;
use std::error::Error;


pub async fn check_health(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/health", base_url);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Health check failed: {}", response.status()).into())
    }
}

pub async fn generate_text(client: &Client, db: &Database, base_url: &str, prompt: &str, max_length: u32) -> Result<String, String> {
    let start_time = Local::now();

    let url = format!("{}/generate", base_url);
    let payload = serde_json::json!({
        "prompt": prompt,
        "max_length": max_length
    });

    let response = match client.post(&url).json(&payload).send().await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => {
                eprintln!("❌ Failed to parse AI response: {}", e);
                return Err(format!("Failed to parse AI response: {}", e));
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to contact AI service: {}", e);
            return Err(format!("Failed to contact AI service: {}", e));
        }
    };

    let end_time = Local::now();
    let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;

    let response_text = response["response"].as_str().unwrap_or("No response").to_string();

    let session_data = AIRequest {
        _id: None,
        timestamp: Local::now().to_rfc3339(),
        prompt: prompt.to_string(),
        max_length,
        response: AIResponse {
            prompt: prompt.to_string(),
            response: response_text.clone(),
            stats: GenerationStats {
                generation_time_seconds: duration,
            },
        },
    };

    if let Err(e) = db.log_request(session_data).await {
        return Err(format!("Failed to store AI response in MongoDB: {}", e));
    }

    Ok(response_text)
}