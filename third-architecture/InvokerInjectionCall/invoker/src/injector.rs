use serde::{Serialize, Deserialize};

use mongodb::{Client, Collection, options::ClientOptions};

use moka::future::Cache;
use std::time::Duration;
use mongodb::bson::{self, doc};
use mongodb::error::Result as MongoResult;




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub ServiceName: String,
    pub ServiceAddress: String,
    pub ServiceTopic: String,
}


#[derive(Clone)]
pub struct ServiceRegistry {
    client: Client,
    cache: Cache<String, Service>,
}



pub async fn get_mongo_client(uri: &str) -> Client {
    let options = ClientOptions::parse(uri).await.unwrap();
    Client::with_options(options).unwrap()
}

pub fn get_collection(client: &Client) -> Collection<Service> {
    let db = client.database("services");
    db.collection::<Service>("services")
}



impl ServiceRegistry {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(60))  // Cache expiry
                .max_capacity(10)                     // Max entries
                .build(),
        }
    }

    /// Insert a new service
    pub async fn insert_service(&self, service: &Service) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        collection.insert_one(service).await?;
        self.cache.insert(service.id.clone(), service.clone()).await;
        Ok(())
    }

    /// Retrieve a service by ID (uses caching)
    pub async fn get_service(&self, id: &str) -> Option<Service> {
        if let Some(service) = self.cache.get(id).await {
            return Some(service);
        }

        let collection = get_collection(&self.client);
        if let Ok(Some(service)) = collection.find_one(doc! { "id": id }).await {
            self.cache.insert(id.to_string(), service.clone()).await;
            return Some(service);
        }
        None
    }

    /// Update a service by ID
    pub async fn update_service(&self, id: &str, service: &Service) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        collection.update_one(doc! { "id": id }, doc! { "$set": bson::to_document(service)? }).await?;
        self.cache.insert(id.to_string(), service.clone()).await;
        Ok(())
    }

    /// Delete a service by ID
    pub async fn delete_service(&self, id: &str) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        collection.delete_one(doc! { "id": id }).await?;
        self.cache.invalidate(&id.to_string()).await;
        Ok(())
    }
}