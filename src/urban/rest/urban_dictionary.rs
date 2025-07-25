use std::env;
use serde::Deserialize;
use crate::urban::UrbanDefinition;
use crate::dictionary::{DictionaryError, DictionaryErrorKind};

#[derive(Debug, Deserialize)]
pub struct UrbanDictionaryResponse {
    pub list: Vec<UrbanDefinition>,

}
pub async fn get_urban_definition(word: &str) -> Result<Vec<UrbanDefinition>, DictionaryError> {
    let urban_dictionary_key = match env::var("URBAN_DICTIONARY_KEY") {
        Ok(val) => val,
        Err(_) => return Err(DictionaryError {
            kind: DictionaryErrorKind::Unknown,
            message: "URBAN_DICTIONARY_KEY is not set".to_string(),
        }),
    };

    if urban_dictionary_key.is_empty() {
        return Err(DictionaryError {
            kind: DictionaryErrorKind::Unknown,
            message: "URBAN_DICTIONARY_KEY is empty".to_string(),
        });
    }

    let url = format!("https://mashape-community-urban-dictionary.p.rapidapi.com/define?term={}", word);
    let client = reqwest::Client::new();
    let request = client.get(&url)
        .header("X-RapidAPI-Host", "mashape-community-urban-dictionary.p.rapidapi.com")
        .header("X-RapidAPI-Key", &urban_dictionary_key);
    let result = request.send().await;

    if result.is_err() {
        return Err(DictionaryError {
            kind: DictionaryErrorKind::Unknown,
            message: "Unknown error occurred".to_string(),
        });
    }
    let response = result.unwrap();

    if response.status().is_client_error() {
        return Err(DictionaryError {
            kind: DictionaryErrorKind::NotFound,
            message: "Word not found".to_string(),
        });
    }

    if response.status().is_server_error() {
        return Err(DictionaryError {
            kind: DictionaryErrorKind::Unknown,
            message: "Unknown server error occurred".to_string(),
        });
    }


    let result_parse = response.json::<UrbanDictionaryResponse>().await;

    match result_parse {
        Ok(result_parse) => Ok(result_parse.list),
        Err(_) => Err(DictionaryError {
            kind: DictionaryErrorKind::Parse,
            message: "Error parsing the response".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_urban_definition() {
        let word = "wheelsucker";
        let result = get_urban_definition(word).await;
        assert!(result.is_ok());

        let definitions = result.unwrap();
        assert!(!definitions.is_empty());

        let definition = &definitions[0];
        assert_eq!(definition.word, word);
        assert_eq!(definition.definition, "A [road cyclist] who stays behind other cyclists' wheels so that he/[she can] draft behind them, and thus [conserve] his/her own efforts.");
        assert_eq!(definition.example, "George is [the worst] wheelsucker of all, always feigning fatigue when we ask him [to be at] the front every [once in a while].");
        assert_eq!(definition.author, "JKu");
        assert_eq!(definition.permalink, "https://www.urbandictionary.com/define.php?term=wheelsucker&defid=1766772");
    }
}
