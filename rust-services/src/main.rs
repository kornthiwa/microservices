mod utils;
mod discord;
mod service;
mod models;
use std::error::Error;
use dotenv::dotenv;
use crate::utils::mongo;
use crate::discord::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    mongo::init().await?;
    client::run().await?;
    
    Ok(())
}
