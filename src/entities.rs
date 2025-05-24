// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
use serde::Serialize;
use serde::de::DeserializeOwned;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Table {
//     pub id: Uuid,
//     pub name: String,
//     pub fully_qualified_name: String,
//     pub columns: Vec<Column>,
//     // ... other fields
// }

/// The Entity trait defines the common interface for all entities in the OpenMetadata ecosystem.
///
/// Possible Entities include
/// - Tables
/// - Databases
/// - Users
pub trait Entity: Serialize + DeserializeOwned {
    /// Returns the type of the entity.
    fn entity_type() -> &'static str;

    /// Default implementation for creating a new entity. Useful for testing or creating default instances.
    fn default() -> Self;
}

// pub struct TableBuilder {
//     name: String,
//     database_schema: String,
//     columns: Vec<Column>,
//     // optional fields...
// }

// impl TableBuilder {
//     pub fn new(name: impl Into<String>) -> Self {}
//     pub fn with_columns(mut self, columns: Vec<Column>) -> Self {}
//     pub fn build(self) -> Result<CreateTableRequest, Error> {
//         Ok(CreateTableRequest {
//             name: self.name,
//             database_schema: self.database_schema,
//             columns: self.columns,
//             // ... other fields
//         })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_table_builder() {
//         let table = TableBuilder::new("test_table")
//             .with_columns(vec![Column::new("id", ColumnType::Int)])
//             .build()
//             .unwrap();

//         assert_eq!(table.name, "test_table");
//         assert_eq!(table.columns.len(), 1);
//     }
// }
