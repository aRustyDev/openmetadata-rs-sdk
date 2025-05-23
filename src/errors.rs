use reqwest;
use serde_json;
use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum OpenMetadataError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Entity not found: {0}")]
    NotFound(String),
    // ... other variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_errors() {
        let e = OpenMetadataError::NotFound("test".to_string());
        assert_eq!(e.to_string(), "Entity not found: test");

        let e = OpenMetadataError::Http("Invalid URL".to_string());
        assert_eq!(e.to_string(), "HTTP error: Invalid URL");

        let e = OpenMetadataError::Serialization("Serialization error".to_string());
        assert_eq!(e.to_string(), "Serialization error: Serialization error");
    }
}
