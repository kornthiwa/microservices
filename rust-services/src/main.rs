mod discord;
mod models;
mod service;
mod utils;
use crate::discord::client;
use crate::utils::mongo;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    mongo::init().await?;
    client::run().await?;

    Ok(())
}
