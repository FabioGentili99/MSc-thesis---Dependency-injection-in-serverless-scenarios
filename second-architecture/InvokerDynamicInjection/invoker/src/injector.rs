use serde::{Serialize, Deserialize};

use mongodb::{Client, Collection, options::ClientOptions};

use mongodb::bson::{self, doc};
use mongodb::error::Result as MongoResult;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub ServiceName: String,
    pub ServiceAddress: String,
}

#[derive(Clone)]
pub struct ServiceRegistry {
    client: Client,
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
        Self { client }
    }

    /// Insert a new service
    pub async fn insert_service(&self, service: &Service) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        collection.insert_one(service).await?;
        Ok(())
    }

    /// Retrieve a service by ID
    pub async fn get_service(&self, id: &str) -> Option<Service> {
        let collection = get_collection(&self.client);
        if let Ok(Some(service)) = collection.find_one(doc! { "id": id }).await {
            return Some(service);
        }
        None
    }

    /// Update a service by ID
    pub async fn update_service(&self, id: &str, service: &Service) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        let service_doc = bson::to_document(service).unwrap();
        collection.update_one(doc! { "id": id }, doc! { "$set": service_doc }).await?;
        Ok(())
    }

    /// Delete a service by ID
    pub async fn delete_service(&self, id: &str) -> MongoResult<()> {
        let collection = get_collection(&self.client);
        collection.delete_one(doc! { "id": id }).await?;
        Ok(())
    }
}
