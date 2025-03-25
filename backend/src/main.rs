mod agent;
mod database;

use std::env;

use reqwest::Client;
use tokio;
use crate::database::Database;
use dotenvy::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_url = env::var("DEEPSEEK_URL").expect("❌ LLM_URL not set in .env");
    let client = Client::new();

    let db = match Database::new().await {
        Ok(database) => database,
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            return;
        }
    };

    match agent::check_health(&client, &api_url).await {
        Ok(_) => println!("✅ AI API is healthy"),
        Err(e) => eprintln!("❌ AI API health check failed: {}", e),
    }
    
    match agent::generate_text(&client, &db, &api_url, "Hello AI!", 100).await {
        Ok(response) => println!("AI Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
