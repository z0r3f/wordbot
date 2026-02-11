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

        match &result {
            Ok(definitions) => {
                println!("✅ API call successful");
                println!("📊 Got {} definitions", definitions.len());
                for (i, def) in definitions.iter().enumerate() {
                    println!("  {}: {}", i + 1, &def.definition[..std::cmp::min(50, def.definition.len())]);
                }
            }
            Err(e) => {
                println!("❌ API call failed: {:?}", e);
            }
        }

        assert!(result.is_ok(), "API call should succeed");

        let definitions = result.unwrap();

        assert!(!definitions.is_empty(), "Should get at least one definition");
        assert!(definitions.len() <= 10, "Should not exceed 10 definitions");

        if definitions.len() != 10 {
            println!("⚠️  Expected 10 definitions but got {}", definitions.len());
        }
    }
}