use reqwest::Client;
use tokio;

mod api;
mod agent;

#[tokio::main]
async fn main() {
    let api_url = "http://localhost:8080"; // Deepseek API
    let client = Client::new();

    // Check if the AI API is running
    match api::check_health(&client, api_url).await {
        Ok(_) => println!("✅ AI API is healthy"),
        Err(e) => {
            eprintln!("❌ AI API health check failed: {}", e);
            return;
        }
    }

    // Example: Send a request
    match agent::generate_text(&client, api_url, "Hello AI!", 100).await {
        Ok(response) => println!("AI Response: {}", response),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
