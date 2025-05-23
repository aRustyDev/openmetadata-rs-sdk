use crate::entities::Entity;
use crate::errors::OpenMetadataError;
use reqwest::Client;
use uuid::Uuid;

// Authentication and connection handling
pub struct OpenMetadataClient {
    base_url: String,
    auth_token: String,
    client: reqwest::Client,
}

impl Entity for OpenMetadataClient {}

impl OpenMetadataClient {
    pub async fn create<T: Entity>(&self, entity: &T) -> Result<(), OpenMetadataError> {
        Ok(())
    }
    pub async fn get_by_id<T: Entity>(&self, id: &Uuid) -> Result<Option<()>, OpenMetadataError> {
        Ok(Some(()))
    }
    pub async fn get_by_name<T: Entity>(&self, fqn: &str) -> Result<Option<()>, OpenMetadataError> {
        Ok(Some(()))
    }
    pub async fn update<T: Entity>(&self, entity: &T) -> Result<(), OpenMetadataError> {
        Ok(())
    }
    pub async fn delete<T: Entity>(&self, id: &Uuid) -> Result<(), OpenMetadataError> {
        Ok(())
    }
}
