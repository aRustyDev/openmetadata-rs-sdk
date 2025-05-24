use crate::crud::Crud;
use crate::entities::Entity;
use crate::errors::OpenMetadataError;
use futures::Future;
use uuid::Uuid;

/// Authentication and connection handling
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenMetadataClient {
    base_url: String,
    auth_token: String,
    // client: reqwest::Client,
}

impl Entity for OpenMetadataClient {
    fn entity_type() -> &'static str {
        "openmetadata_client"
    }
    fn default() -> Self {
        Self {
            base_url: String::new(),
            auth_token: String::new(),
        }
    }
}

impl Crud for OpenMetadataClient {
    type Entity = OpenMetadataClient;

    fn create<T: Entity>(
        &self,
        _entity: &T,
    ) -> impl Future<Output = Result<Uuid, OpenMetadataError>> {
        async move { Ok(Uuid::new_v4()) }
    }

    fn read<T: Entity>(
        &self,
        _id: &Uuid,
    ) -> impl Future<Output = Result<Option<T>, OpenMetadataError>> {
        async move { Ok(Some(T::default())) }
    }

    fn update<T: Entity>(
        &self,
        _id: &Uuid,
        _entity: &T,
    ) -> impl Future<Output = Result<(), OpenMetadataError>> {
        async move { Ok(()) }
    }

    fn delete<T: Entity>(&self, _id: &Uuid) -> impl Future<Output = Result<(), OpenMetadataError>> {
        async move { Ok(()) }
    }
}
