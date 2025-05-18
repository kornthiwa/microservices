use crate::models::channels::Channel;
use crate::utils::mongo;
use mongodb::bson::{doc, oid::ObjectId};

pub struct ChannelsService;

impl ChannelsService {
    // Get MongoDB collection
    pub async fn get_collection() -> mongodb::Collection<Channel> {
        let db_pool: &'static mongo::MongoPool = mongo::get_pool().await;
        db_pool.collection::<Channel>("channels")
    }

    pub async fn create_channel(channel: Channel) -> Result<ObjectId, mongodb::error::Error> {
        let channels_collection: mongodb::Collection<Channel> = Self::get_collection().await;
        let result = channels_collection.insert_one(channel).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    pub async fn get_channels_by_guild(
        guild_id: &str,
    ) -> Result<Vec<Channel>, mongodb::error::Error> {
        let channels_collection: mongodb::Collection<Channel> = Self::get_collection().await;
        let mut cursor = channels_collection
            .find(doc! { "guild_id": guild_id })
            .await?;

        let mut channels = Vec::new();
        while cursor.advance().await? {
            channels.push(cursor.deserialize_current()?);
        }

        Ok(channels)
    }
}
