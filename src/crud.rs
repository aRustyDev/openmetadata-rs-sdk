use crate::entities::Entity;
use crate::errors::OpenMetadataError;
use uuid::Uuid;

/// Required operations for entities to be CRUD compatible.
pub trait Crud {
    type Entity: Entity;

    /// Create a new entity.
    async fn create(&self, entity: &Self::Entity) -> Result<Uuid, OpenMetadataError>;

    /// Read an entity by its ID.
    async fn read(&self, id: &Uuid) -> Result<Option<Self::Entity>, OpenMetadataError>;

    /// Update an entity by its ID.
    async fn update(&self, id: &Uuid, entity: &Self::Entity) -> Result<(), OpenMetadataError>;

    /// Delete an entity.
    async fn delete(&self, id: &Uuid) -> Result<(), OpenMetadataError>;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     async fn test_create_entity() {
//         let client = OpenMetadataClient::new("http://localhost:8080");
//         let entity = Entity::new("test_entity");

//         let result = client.create(&entity).await;
//         assert!(result.is_ok());
//     }

//     #[test]
//     async fn test_get_by_id() {
//         let client = OpenMetadataClient::new("http://localhost:8080");
//         let entity = Entity::new("test_entity");

//         let result = client.create(&entity).await;
//         assert!(result.is_ok());

//         let id = result.unwrap().id;
//         let result = client.get_by_id::<Entity>(&id).await;
//         assert!(result.is_ok());
//         assert!(result.unwrap().is_some());
//     }
// }
