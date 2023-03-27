mod rest;

use std::fmt;
use serde::Deserialize;
use crate::dictionary::rest::api_dictionary::get_definition;

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub(crate) word: String,
    pub(crate) meanings: Vec<Meaning>,
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Word: {}", self.word)?;
        writeln!(f, "Meanings:")?;
        for meaning in &self.meanings {
            writeln!(f, "{}", meaning)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    #[serde(rename(deserialize = "partOfSpeech"))]
    pub(crate) part_of_speech: String,
    pub(crate) definitions: Vec<DefinitionDetail>,
}

impl fmt::Display for Meaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Part of speech: {}", self.part_of_speech)?;
        writeln!(f, "Definitions:")?;
        for definition in &self.definitions {
            writeln!(f, "{}", definition)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct DefinitionDetail {
    pub(crate) definition: String,
    pub(crate) antonyms: Option<Vec<String>>,
    pub(crate) synonyms: Option<Vec<String>>,
    pub(crate) example: Option<String>,
}

impl fmt::Display for DefinitionDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Definition: {}", self.definition)?;
        if let Some(antonyms) = &self.antonyms {
            writeln!(f, "Antonyms: {:?}", antonyms)?;
        }
        if let Some(synonyms) = &self.synonyms {
            writeln!(f, "Synonyms: {:?}", synonyms)?;
        }
        if let Some(example) = &self.example {
            writeln!(f, "Example: {}", example)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DictionaryError {
    pub kind: DictionaryErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DictionaryErrorKind {
    NotFound,
    Parse,
    Unknown,
}

pub async fn definition(word: &str) -> Result<Vec<Definition>, DictionaryError> {
    get_definition(word).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn definition_should_data() {
        let word = "mouse";

        let result = definition(word).await;
        assert!(result.is_ok());

        let definitions = result.unwrap();
        assert!(!definitions.is_empty());

        let definition = &definitions[0];
        assert_eq!(definition.word, word);
    }
}
