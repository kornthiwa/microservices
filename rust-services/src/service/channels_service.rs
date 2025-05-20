use crate::models::channels::Channel;
use crate::utils::mongo;
use mongodb::bson::{doc, oid::ObjectId};
use futures::TryStreamExt;


pub struct ChannelsService;

impl ChannelsService {
    // Create a new instance of ChannelsService
    pub async fn get_collection() -> mongodb::Collection<Channel> {
        let db_pool: &'static mongo::MongoPool = mongo::get_pool().await;
        db_pool.collection::<Channel>("channels")
    }

    pub async fn create_channel(channel: Channel) -> Result<ObjectId, mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let result = collection.insert_one(channel).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    pub async fn update_channel( channel: &Channel) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let now = mongodb::bson::DateTime::from(std::time::SystemTime::now());

        collection
            .update_one(
                doc! {
                    "guild_id": &channel.guild_id
                },
                doc! {
                    "$set": {
                        "channel_name": &channel.channel_name,
                        "channel_id": &channel.channel_id,
                        "guild_name": &channel.guild_name,
                        "updated_at": now
                    }
                },
            )
            .await?;
        Ok(())
    }

    pub async fn get_all_channels() -> Result<Vec<Channel>, mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let mut cursor = collection.find(doc! {}).await?;
        let mut channels = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            channels.push(doc);
        }
        Ok(channels)
    }

    pub async fn get_channels_by_guild(guild_id: &str) -> Result<Vec<Channel>, mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let mut cursor = collection.find(doc! { "guild_id": guild_id }).await?;
        let mut channels = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            channels.push(doc);
        }
        Ok(channels)
    }
    
}
