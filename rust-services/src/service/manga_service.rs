use crate::models::manga::Manga;
use crate::utils::mongo;
use futures::TryStreamExt;
use mongodb::bson::doc;

pub struct MangaService;

#[allow(dead_code)]
impl MangaService {
    pub async fn get_collection() -> mongodb::Collection<Manga> {
        let db_pool: &'static mongo::MongoPool = mongo::get_pool().await;
        db_pool.collection::<Manga>("mangas")
    }

    pub async fn create(manga: &Manga) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_collection().await;
        collection.insert_one(manga).await?;
        Ok(())
    }

    pub async fn get_by_url(url: &str) -> Result<Option<Manga>, mongodb::error::Error> {
        let collection: mongodb::Collection<Manga> = Self::get_collection().await;
        collection.find_one(doc! { "url": url }).await
    }

    pub async fn update(manga: &Manga) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let now = mongodb::bson::DateTime::from(std::time::SystemTime::now());

        let update_doc = doc! {
            "$set": {
                "title": &manga.title,
                "latest_chapter": &manga.latest_chapter,
                "latest_chapter_url": &manga.latest_chapter_url,
                "image_url": &manga.image_url,
                "updated_at": now
            }
        };

        collection
            .update_one(doc! { "url": &manga.url }, update_doc)
            .await?;
        Ok(())
    }

    pub async fn get_all() -> Result<Vec<Manga>, mongodb::error::Error> {
        let collection = Self::get_collection().await;
        let mut cursor = collection.find(doc! {}).await?;
        let mut mangas = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            mangas.push(doc);
        }
        Ok(mangas)
    }
}
