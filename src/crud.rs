use crate::connection::OpenMetadataClient;
use crate::errors::OpenMetadataError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Entity: Serialize + DeserializeOwned {
    fn entity_type() -> &'static str;
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
