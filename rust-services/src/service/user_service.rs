use mongodb::bson::{doc};
use crate::models::user::User;
use crate::utils::mongo;

pub struct UserService;

impl UserService {
    // ฟังก์ชั่นดึง MongoDB collection
    pub async fn get_collection() -> mongodb::Collection<User> {
        let db_pool: &'static mongo::MongoPool = mongo::get_pool().await;
        db_pool.collection::<User>("users")
    }
    
    // ลงทะเบียนผู้ใช้ใหม่
    pub async fn register_user(guild_id: &str, user_id: &str, user_name: &str) -> Result<(), mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;
        let user: User = User::new(
            user_id.to_string(),
            guild_id.to_string(),
            user_name.to_string()
        );
        
        users_collection.insert_one(user).await?;
        
        Ok(())
    }
    
    // ค้นหาผู้ใช้จาก user_id
    pub async fn find_by_user_id(user_id: &str, guild_id: &str) -> Result<Option<User>, mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;
        
        users_collection.find_one(doc! {
            "user_id": user_id,
            "guild_id": guild_id
        }).await
    }

    //เช็คว่ามีผู้ใช้งานหรือยัง
    pub async fn check_user_exists(user_id: &str, guild_id: &str) -> Result<bool, mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;
        
        let user: Option<User> = users_collection.find_one(doc! {
            "user_id": user_id,
            "guild_id": guild_id
        }).await?;
        
        Ok(user.is_some())
    }
}