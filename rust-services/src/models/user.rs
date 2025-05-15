use chrono::{DateTime, Utc};
use serde::{ Deserialize, Serialize };
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub guild_id: String,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(user_id: String, guild_id: String, user_name: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        User {
            id: None,
            user_id,
            guild_id,
            user_name,
            created_at: now,
            updated_at: now,
        }
    }
}
