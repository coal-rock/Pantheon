use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables
    dotenv().ok();

    // Get Redis connection details from environment or default
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);

    // Connect to Redis
    let client = Client::open(redis_url)?;
    let mut con = client.get_multiplexed_async_connection().await?;

    // Example data to store
    let user = User {
        id: 1,
        name: "Hermes".to_string(),
        email: "hermes@olympus.org".to_string(),
    };

    // Serialize and store data in Redis
    let key = format!("user:{}", user.id);
    let user_data = serde_json::to_string(&user)?;
    let _: () = con.set(&key, user_data).await?;
    println!("User data stored in Redis with key: {}", key);

    // Retrieve and deserialize the data
    let stored_data: String = con.get(&key).await?;
    let retrieved_user: User = serde_json::from_str(&stored_data)?;
    println!("Retrieved user data: {:?}", retrieved_user);

    Ok(())
}

