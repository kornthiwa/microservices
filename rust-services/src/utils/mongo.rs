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

static MONGO: OnceCell<MongoPool> = OnceCell::const_new();

pub async fn init() -> mongodb::error::Result<()> {
    let mongo_uri: String = std::env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let db_name: String = std::env::var("MONGO_DB_NAME")
        .unwrap_or_else(|_| "discord_bot".to_string());
    
    println!("กำลังเชื่อมต่อ MongoDB ที่: {}", mongo_uri);
    
    let mut options: ClientOptions = ClientOptions::parse(&mongo_uri).await?;
    options.max_pool_size = Some(100);
    options.min_pool_size = Some(5);
    options.max_idle_time = Some(Duration::from_secs(60));
    options.server_selection_timeout = Some(Duration::from_secs(5));
    
    let client: Client = Client::with_options(options)?;
    
    match client.list_database_names().await {
        Ok(_) => println!("ทดสอบเชื่อมต่อ MongoDB สำเร็จ"),
        Err(e) => {
            eprintln!("ไม่สามารถเชื่อมต่อ MongoDB: {}", e);
            eprintln!("กรุณาตรวจสอบว่า MongoDB server กำลังทำงานอยู่");
            return Err(e.into());
        }
    }
    
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