use crate::entities::Entity;
use crate::errors::OpenMetadataError;
use futures::future::Future;
use uuid::Uuid;

/// Required operations for entities to be CRUD compatible.
pub trait Crud {
    type Entity: Entity;

    /// Create a new entity.
    fn create<T: Entity>(
        &self,
        entity: &T,
    ) -> impl Future<Output = Result<Uuid, OpenMetadataError>> + Send;

    /// Read an entity by its ID.
    fn read<T: Entity>(
        &self,
        id: &Uuid,
    ) -> impl Future<Output = Result<Option<T>, OpenMetadataError>> + Send;

    /// Update an entity by its ID.
    fn update<T: Entity>(
        &self,
        id: &Uuid,
        entity: &T,
    ) -> impl Future<Output = Result<(), OpenMetadataError>> + Send;

    /// Delete an entity.
    fn delete<T: Entity>(
        &self,
        id: &Uuid,
    ) -> impl Future<Output = Result<(), OpenMetadataError>> + Send;
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
