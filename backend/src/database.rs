use std::env;

use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;


#[derive(Debug, Serialize, Deserialize)]
pub struct AIRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<mongodb::bson::oid::ObjectId>,
    pub timestamp: String,
    pub prompt: String,
    pub max_length: u32,
    pub response: AIResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub prompt: String,
    pub response: String,
    pub stats: GenerationStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationStats {
    pub generation_time_seconds: f64,
}

pub struct Database {
    client: Client,
}

impl Database {
    /// Connect to MongoDB, returning an error if the connection fails.
    pub async fn new() -> Result<Self, String> {
        dotenv().ok();

        let uri = env::var("MONGO_URI").expect("❌ MONGO_URI not set in .env");
        match ClientOptions::parse(uri).await {
            Ok(options) => match Client::with_options(options) {
                Ok(client) => {
                    println!("✅ Successfully connected to MongoDB.");
                    Ok(Self { client })
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize MongoDB client: {}", e);
                    Err(format!("Failed to initialize MongoDB client: {}", e))
                }
            },
            Err(e) => {
                eprintln!("❌ Failed to parse MongoDB URI: {}", e);
                Err(format!("Failed to parse MongoDB URI: {}", e))
            }
        }
    }

    /// Insert request into MongoDB and return an error if the transaction fails.
    pub async fn log_request(&self, request: AIRequest) -> Result<(), String> {
        dotenv().ok();

        let db_name = env::var("MONGO_DB_NAME").expect("❌ DB_NAME not set in .env");
        let col_name = env::var("MONGO_COLLECTION_NAME").expect("❌ COLLECTION_NAME not set in .env");

        let db = self.client.database(&db_name);
        let collection: Collection<AIRequest> = db.collection(&col_name);

        match collection.insert_one(request, None).await {
            Ok(_) => {
                println!("✅ Successfully stored request in MongoDB.");
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to insert document: {}", e);
                Err(format!("Failed to insert document: {}", e))
            }
        }
    }
}
