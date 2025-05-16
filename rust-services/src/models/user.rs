use serde::{ Deserialize, Serialize };
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub guild_id: String,
    pub guild_name: String,
    pub guild_user_nickname: String,
    pub global_name: String,
    pub user_name: String,
    pub created_at: mongodb::bson::DateTime,
    pub updated_at: mongodb::bson::DateTime,
}

impl User {
    pub fn new(user_id: String, guild_id: String, guild_name: String, guild_user_nickname: String, global_name: String, user_name: String) -> Self {
        let now: mongodb::bson::DateTime = mongodb::bson::DateTime::from(std::time::SystemTime::now());
        User {
            id: None,
            user_id,
            guild_id,
            guild_name,
            guild_user_nickname,
            global_name,
            user_name,
            created_at: now,
            updated_at: now,
        }
    }
}
