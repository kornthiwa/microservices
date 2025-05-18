use crate::models::user::User;
use crate::utils::mongo;
use mongodb::bson::doc;

pub struct UserService;

impl UserService {
    // ฟังก์ชั่นดึง MongoDB collection
    pub async fn get_collection() -> mongodb::Collection<User> {
        let db_pool: &'static mongo::MongoPool = mongo::get_pool().await;
        db_pool.collection::<User>("users")
    }

    // ลงทะเบียนผู้ใช้ใหม่
    pub async fn register_user(
        guild_id: &str,
        user_id: &str,
        guild_name: &str,
        guild_user_nickname: &str,
        global_name: &str,
        user_name: &str,
    ) -> Result<(), mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;
        let user: User = User::new(
            user_id.to_string(),
            guild_id.to_string(),
            guild_name.to_string(),
            guild_user_nickname.to_string(),
            global_name.to_string(),
            user_name.to_string(),
        );

        users_collection.insert_one(user).await?;

        Ok(())
    }

    // ค้นหาผู้ใช้จาก user_id
    pub async fn find_by_user_id(
        user_id: &str,
        guild_id: &str,
    ) -> Result<Option<User>, mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;

        users_collection
            .find_one(doc! {
                "user_id": user_id,
                "guild_id": guild_id
            })
            .await
    }

    //เช็คว่ามีผู้ใช้งานหรือยัง
    pub async fn check_user_exists(
        user_id: &str,
        guild_id: &str,
    ) -> Result<bool, mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;

        let user: Option<User> = users_collection
            .find_one(doc! {
                "user_id": user_id,
                "guild_id": guild_id
            })
            .await?;

        Ok(user.is_some())
    }

    pub async fn update_user(user: &User) -> Result<(), mongodb::error::Error> {
        let users_collection: mongodb::Collection<User> = Self::get_collection().await;
        let now = mongodb::bson::DateTime::from(std::time::SystemTime::now());

        users_collection
            .update_one(
                doc! {
                    "user_id": &user.user_id,
                    "guild_id": &user.guild_id
                },
                doc! {
                    "$set": {
                        "guild_user_nickname": &user.guild_user_nickname,
                        "global_name": &user.global_name,
                        "user_name": &user.user_name,
                        "updated_at": now
                    }
                },
            )
            .await?;
        Ok(())
    }
}
