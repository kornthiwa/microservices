use serenity::all::{
    CreateCommand, CommandInteraction, Context,
    CreateInteractionResponse, CreateInteractionResponseMessage,
    CommandOptionType, CreateCommandOption
};
use crate::service::user_service::UserService;

pub fn register() -> CreateCommand {
    CreateCommand::new("user")
        .description("จัดการข้อมูลผู้ใช้")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "info",
                "ดูข้อมูลของผู้ใช้"
            )
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "register",
                "ลงทะเบียนผู้ใช้"
            )
        )
}

// ฟังก์ชันย่อยสำหรับลงทะเบียนผู้ใช้
async fn register_user(command: &CommandInteraction) -> String {
    let guild_id: String = command.guild_id.unwrap().to_string();
    let user_id: String = command.user.id.to_string();
    let user_name: String = command.user.name.clone();
    
    // ตรวจสอบว่าผู้ใช้มีอยู่แล้วหรือไม่
    match UserService::check_user_exists(&user_id, &guild_id).await {
        Ok(true) => format!("ผู้ใช้ {} ได้ลงทะเบียนไว้แล้ว", user_name),
        Ok(false) => {
            // ลงทะเบียนเมื่อยังไม่มีข้อมูลผู้ใช้
            match UserService::register_user(&guild_id, &user_id, &user_name).await {
                Ok(_) => format!("ลงทะเบียนผู้ใช้ {} สำเร็จ", user_name),
                Err(e) => format!("เกิดข้อผิดพลาด: {}", e),
            }
        },
        Err(e) => format!("เกิดข้อผิดพลาดในการตรวจสอบข้อมูลผู้ใช้: {}", e),
    }
}

// ฟังก์ชันย่อยสำหรับดูข้อมูลผู้ใช้
async fn get_user_info(command: &CommandInteraction) -> String {
    let user_id: String = command.user.id.to_string();
    let guild_id: String = command.guild_id.unwrap().to_string();
    
    match UserService::find_by_user_id(&user_id, &guild_id).await {
        Ok(Some(user)) => format!("ข้อมูลผู้ใช้: {}", user.user_name),
        Ok(None) => "ไม่พบข้อมูลผู้ใช้".to_string(),
        Err(e) => format!("เกิดข้อผิดพลาด: {}", e),
    }
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    _: &serenity::prelude::TypeMap
) -> serenity::Result<()> {
    // ดึงข้อมูล subcommand
    let subcommand: &serenity::all::CommandDataOption = command.data.options.first().unwrap();
    let subcommand_name: &String = &subcommand.name;
    
    let response: String = match subcommand_name.as_str() {
        "info" => get_user_info(command).await,
        "register" => register_user(command).await,
        _ => "ไม่รู้จักคำสั่งย่อย".to_string()
    };
    
    command.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(response)
                .ephemeral(true)
        )
    ).await
}