use std::env;
use serenity::all::{Client, GatewayIntents};
use crate::discord::handlers::Handlers;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("ไม่พบ DISCORD_TOKEN");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    println!("กำลังเริ่มต้นบอท Discord...");

    let mut client = Client::builder(&token, intents)
        .event_handler(Handlers)
        .await?;

    println!("บอทพร้อมทำงานแล้ว กำลังเชื่อมต่อกับ Discord...");

    if let Err(why) = client.start().await {
        println!("เกิดข้อผิดพลาดกับไคลเอนต์: {why:?}");
    }
    
    Ok(())
}