use mongodb::{Client, Database, Collection, options::ClientOptions};
use std::sync::Arc;
use tokio::sync::OnceCell;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct MongoPool {
    client: Arc<Client>,
    default_db: String,
}

impl MongoPool {
    pub fn default_database(&self) -> Database {
        self.client.database(&self.default_db)
    }

    pub fn collection<T: Send + Sync>(&self, collection_name: &str) -> Collection<T> {
        self.default_database().collection(collection_name)
    }
}

// Global instance
static MONGO: OnceCell<MongoPool> = OnceCell::const_new();

pub async fn init() -> mongodb::error::Result<()> {
    let mongo_uri: String = std::env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let db_name: String = std::env::var("MONGO_DB_NAME")
        .unwrap_or_else(|_| "discord_bot".to_string());
    
    let mut options: ClientOptions = ClientOptions::parse(mongo_uri).await?;
    options.max_pool_size = Some(100);
    options.min_pool_size = Some(5);
    options.max_idle_time = Some(Duration::from_secs(60));
    
    let client: Client = Client::with_options(options)?;
    let pool: MongoPool = MongoPool {
        client: Arc::new(client),
        default_db: db_name,
    };
    
    MONGO.set(pool).unwrap();
    println!("เชื่อมต่อ MongoDB สำเร็จ");
    Ok(())
}

pub async fn get_pool() -> &'static MongoPool {
    MONGO.get().expect("MongoDB ยังไม่ได้เริ่มต้น")
}