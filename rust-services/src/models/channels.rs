use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub channel_id: String,
    pub guild_id: String,
    pub guild_name: String,
    pub channel_name: String,
    pub created_at: mongodb::bson::DateTime,
    pub updated_at: mongodb::bson::DateTime,
}

impl Channel {
    pub fn new(
        channel_id: String,
        guild_id: String,
        channel_name: String,
        guild_name: String,
    ) -> Self {
        let now: mongodb::bson::DateTime =
            mongodb::bson::DateTime::from(std::time::SystemTime::now());
        Channel {
            id: None,
            channel_id,
            guild_id,
            guild_name,
            channel_name,
            created_at: now,
            updated_at: now,
        }
    }
}
