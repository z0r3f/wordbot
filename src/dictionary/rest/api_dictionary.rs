
use crate::dictionary::{Definition, DictionaryError, DictionaryErrorKind};

pub async fn get_definition(word: &str) -> Result<Vec<Definition>, DictionaryError> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let result = reqwest::get(&url).await;

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

    let result_parse = response.json::<Vec<Definition>>().await;

    match result_parse {
        Ok(definitions) => Ok(definitions),
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
    async fn test_get_hello_definition() {
        let word = "hello";
        let result = get_definition(word).await;
        assert!(result.is_ok());

        let definitions = result.unwrap();
        assert!(!definitions.is_empty());

        let definition = &definitions[0];
        assert_eq!(definition.word, word);

        assert!(definition.meanings.iter().any(|m| m.definitions.iter().any(|d| d.definition.contains("A greeting"))));
    }

    #[tokio::test]
    async fn test_get_undefined_word() {
        let word = "sdfkjnsdfkjn";
        let result = get_definition(word).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind, DictionaryErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_get_word_with_spaces() {
        let word = "hello world";
        let result = get_definition(word).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind, DictionaryErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_get_word_with_special_characters() {
        let word = "héllo";
        let result = get_definition(word).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind, DictionaryErrorKind::NotFound);
    }
}
