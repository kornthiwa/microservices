use serenity::async_trait;
use serenity::all::{
    Context, EventHandler, Interaction, Ready,
    Command, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use crate::discord::commands;

pub struct Handlers;

#[async_trait]
impl EventHandler for Handlers {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let data_read: tokio::sync::RwLockReadGuard<'_, serenity::prelude::TypeMap> = ctx.data.read().await;
            let result: Result<(), serenity::Error> = match command.data.name.as_str() {
                "user" => commands::user_commands::run(&ctx, &command, &data_read).await,
                _ => {
                    // ไม่พบคำสั่ง
                    command.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("ไม่พบคำสั่งนี้")
                                .ephemeral(true),
                        ),
                    ).await.ok();
                    Ok(())
                }
            };
            
            // จัดการข้อผิดพลาด
            if let Err(why) = result {
                println!("เกิดข้อผิดพลาดในคำสั่ง '{}': {:?}", command.data.name, why);
                
                // ส่งข้อความแจ้งข้อผิดพลาด
                let _ = command.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("เกิดข้อผิดพลาดในการประมวลผลคำสั่ง")
                            .ephemeral(true),
                    ),
                ).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} เชื่อมต่อแล้ว!", ready.user.name);
        
        // ลงทะเบียนคำสั่งทั้งหมด
        let commands: Vec<serenity::all::CreateCommand> = collect_all_commands();

        // ลงทะเบียนคำสั่งทั้งหมดแบบ global
        match Command::set_global_commands(&ctx.http, commands).await {
            Ok(_) => println!("ลงทะเบียนคำสั่งทั้งหมดสำเร็จ"),
            Err(why) => println!("ลงทะเบียนคำสั่งล้มเหลว: {:?}", why),
        }
    }
}

// ฟังก์ชั่นรวบรวมคำสั่งทั้งหมด
fn collect_all_commands() -> Vec<serenity::all::CreateCommand> {
    vec![
        commands::user_commands::register(),
    ]
}