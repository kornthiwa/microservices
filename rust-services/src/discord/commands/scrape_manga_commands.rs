use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;

pub async fn scrape_manga_sing_manga(
    url: &str,
) -> Result<(String, i32, String, Option<String>), Box<dyn std::error::Error + Send + Sync>> {
    // Check if URL is from sing-manga.com
    if !url.contains("sing-manga.com") {
        return Err("URL ต้องเป็นของ sing-manga.com เท่านั้น".into());
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()?;

    let response = client.get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .send()
        .await?;

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let title_selector = Selector::parse("h1.entry-title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .ok_or("ไม่พบชื่อการ์ตูน")?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let image_selector = Selector::parse("div.thumb img").unwrap();
    let image_url = document
        .select(&image_selector)
        .next()
        .map(|img| img.value().attr("src").unwrap_or_default().to_string());

    let chapter_selector = Selector::parse("div.lastend div.inepcx a").unwrap();
    let chapter_elements: Vec<_> = document.select(&chapter_selector).collect();

    let latest_chapter_element = chapter_elements.last().ok_or("ไม่พบตอนล่าสุด")?;

    let chapter_number_selector = Selector::parse("span.epcurlast").unwrap();
    let chapter_text = latest_chapter_element
        .select(&chapter_number_selector)
        .next()
        .ok_or("ไม่พบหมายเลขตอน")?
        .text()
        .collect::<String>();

    let chapter_number = chapter_text
        .replace("Chapter ", "")
        .trim()
        .replace(|c: char| !c.is_numeric(), "")
        .parse::<i32>()?;

    let chapter_url = latest_chapter_element
        .value()
        .attr("href")
        .ok_or("ไม่พบ URL ของตอนล่าสุด")?
        .to_string();

    // println!("\n=== Scraping Complete ===");
    // println!("Final results:");
    // println!("Title: {}", title);
    // println!("Image URL: {:?}", image_url);
    // println!("Chapter: {}", chapter_number);
    // println!("URL: {}", chapter_url);
    // println!("========================\n");

    Ok((title, chapter_number, chapter_url, image_url))
}
