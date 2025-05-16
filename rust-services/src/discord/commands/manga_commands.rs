use serenity::all::{
    CreateCommand, CommandInteraction, Context,
    CreateInteractionResponse, CreateInteractionResponseMessage,
    CommandOptionType, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
    Colour
};
use crate::service::manga_service::MangaService;
use crate::models::manga::Manga;

pub fn register() -> CreateCommand {
    CreateCommand::new("manga")
        .description("จัดการการ์ตูน")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "add",
                "เพิ่มการ์ตูนใหม่"
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "url",
                    "URL ของการ์ตูนที่ต้องการเพิ่ม"
                )
                .required(true)
            )
        )
}

// UI Utility Function
pub async fn show_manga_info_ui(
    command: &CommandInteraction,
    ctx: &Context,
    title: &str,
    description: &str,
    color: Colour,
) -> serenity::Result<()> {
    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .footer(CreateEmbedFooter::new("ระบบจัดการการ์ตูน"));

    command.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(true)
        )
    ).await
}

async fn add_manga(ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
    let url: &str = command
        .data
        .options
        .iter()
        .find(|opt: &&serenity::all::CommandDataOption| opt.name == "add")
        .and_then(|opt: &serenity::all::CommandDataOption| match &opt.value {
            serenity::all::CommandDataOptionValue::SubCommand(sub_opts) => Some(sub_opts),
            _ => None,
        })
        .and_then(|sub_opts: &Vec<serenity::all::CommandDataOption>| sub_opts.iter().find(|opt: &&serenity::all::CommandDataOption| opt.name == "url"))
        .and_then(|opt: &serenity::all::CommandDataOption| opt.value.as_str())
        .unwrap_or("ไม่มีข้อความ");

    if !url.starts_with("https://www") {
        return show_manga_info_ui(
            command,
            ctx,
            "URL ไม่ถูกต้อง",
            "URL ต้องขึ้นต้นด้วย https://www",
            Colour::RED,
        ).await;
    }

    // ตรวจสอบว่ามีการ์ตูนนี้ในฐานข้อมูลหรือไม่
    match MangaService::get_by_url(url).await {
        Ok(Some(_)) => {
            show_manga_info_ui(
                command,
                ctx,
                "การเพิ่มการ์ตูน",
                "การ์ตูนนี้มีอยู่ในระบบแล้ว",
                Colour::GOLD,
            ).await
        }
        Ok(None) => {
            // สร้างข้อมูลการ์ตูนใหม่
            let manga: Manga = Manga::new(
                "Untitled".to_string(), // ตั้งชื่อชั่วคราว
                url.to_string(),
                0, // เริ่มต้นที่ตอนที่ 0
                url.to_string(), // ใช้ URL เดิมเป็น chapter URL
            );

            // บันทึกลงฐานข้อมูล
            match MangaService::create(&manga).await {
                Ok(_) => {
                    let description = format!(
                        "**เพิ่มการ์ตูนสำเร็จ**\n\
                        **ชื่อเรื่อง:** {}\n\
                        **ตอนล่าสุด:** {}\n\
                        **URL:** {}",
                        manga.title,
                        manga.latest_chapter,
                        manga.url
                    );

                    show_manga_info_ui(
                        command,
                        ctx,
                        "เพิ่มการ์ตูนสำเร็จ",
                        &description,
                        Colour::DARK_GREEN,
                    ).await
                }
                Err(e) => {
                    show_manga_info_ui(
                        command,
                        ctx,
                        "เกิดข้อผิดพลาด",
                        &format!("เกิดข้อผิดพลาดในการบันทึกข้อมูล: {}", e),
                        Colour::RED,
                    ).await
                }
            }
        }
        Err(e) => {
            show_manga_info_ui(
                command,
                ctx,
                "เกิดข้อผิดพลาด",
                &format!("เกิดข้อผิดพลาดในการตรวจสอบข้อมูล: {}", e),
                Colour::RED,
            ).await
        }
    }
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    _: &serenity::prelude::TypeMap
) -> serenity::Result<()> {
    let subcommand = command.data.options.first().unwrap();
    let subcommand_name = &subcommand.name;

    match subcommand_name.as_str() {
        "add" => add_manga(ctx, command).await,
        _ => {
            show_manga_info_ui(
                command,
                ctx,
                "ไม่รู้จักคำสั่ง",
                "ไม่รู้จักคำสั่งย่อยนี้",
                Colour::RED,
            ).await
        }
    }
} 