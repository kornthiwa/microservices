use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manga {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub url: String,
    pub latest_chapter: i32,
    pub latest_chapter_url: String,
    pub image_url: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[allow(dead_code)]
impl Manga {
    pub fn new(
        title: String,
        url: String,
        latest_chapter: i32,
        latest_chapter_url: String,
        image_url: Option<String>,
    ) -> Self {
        let now = DateTime::from(std::time::SystemTime::now());
        Manga {
            id: None,
            title,
            url,
            latest_chapter,
            latest_chapter_url,
            image_url,
            created_at: now,
            updated_at: now,
        }
    }
}
