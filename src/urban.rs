use serde::Deserialize;

use crate::dictionary::DictionaryError;
use crate::urban::rest::urban_dictionary::get_urban_definition;

mod rest;

#[derive(Debug, Deserialize)]
pub struct UrbanDefinition {
    pub word: String,
    pub definition: String,
    pub example: String,
    pub author: String,
    pub permalink: String,
}
pub async fn definition(word: &str) -> Result<Vec<UrbanDefinition>, DictionaryError> {
    get_urban_definition(word).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hello_definition() {
        let word = "hello";
        let result = definition(word).await;
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.len(), 10);
    }
}