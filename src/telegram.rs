use crate::dictionary::Definition;
use crate::urban::UrbanDefinition;

pub trait Sanitize {
    fn sanitize(&self) -> String;
}

impl Sanitize for String {
    fn sanitize(&self) -> String {
        let special_chars = [
            "[", "]", "(", ")", ">", "#", "+", "-", "=", "|", "{", "}", ".", "!", "_", "*", "~", "`",
        ];
        let mut sanitized = self.clone();

        for c in &special_chars {
            sanitized = sanitized.replace(*c, &format!("\\{}", c));
        }

        sanitized
    }
}

pub trait Telegram {
    fn to_message(&self) -> String {
        let mut message = String::new();
        message.push_str(&self.build_message());
        message
    }

    fn build_message(&self) -> String;
}

impl Telegram for Vec<Definition> {
    fn build_message(&self) -> String {
        let mut message = String::new();
        for (_, definition) in self.iter().enumerate() {
            message.push_str(&format!("*Definitions for* _{}_:\n", definition.word.sanitize()));
            message.push_str(&definition.build_message());
        }
        message
    }
}


impl Telegram for Definition {
    fn build_message(&self) -> String {
        let mut message = String::new();
        for meaning in &self.meanings {
            message.push_str(&format!("*[{}]*\n", meaning.part_of_speech.sanitize()));
            for definition in &meaning.definitions {
                message.push_str(&format!("- {}\n", definition.definition.sanitize()));
            }
            message.push_str("\n");
        }
        message
    }
}

impl Telegram for Vec<UrbanDefinition> {
    fn build_message(&self) -> String {
        let mut message = String::new();
        let custom = if self.len() > 1 { "s" } else { "" };
        message.push_str(&format!("Found {} urban definition{} for *{}*\n", self.len(), custom, self[0].word.sanitize()));
        for (_, definition) in self.iter().enumerate() {
            message.push_str(&definition.build_message());
        }
        message
    }
}

impl Telegram for UrbanDefinition {
    fn build_message(&self) -> String {
        let mut message = String::new();
        message.push_str(&format!("\n*Definition:*\n{}\n", self.definition.sanitize()));
        if !self.example.is_empty() {
            message.push_str(&format!("*Example:*\n{}\n", self.example.sanitize()));
        }
        message.push_str(&format!("*Author:*\n{}\n", self.author.sanitize()));
        message
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::{DefinitionDetail, Meaning};
    use super::*;

    #[test]
    fn test_sanitize() {
        let original = "hello-world!".to_string();
        let expected = "hello\\-world\\!".to_string();
        let sanitized = original.sanitize();
        assert_eq!(sanitized, expected);
    }

    #[test]
    fn test_sanitize_with_special_chars() {
        let original = "####()!!".to_string();
        let expected = "\\#\\#\\#\\#\\(\\)\\!\\!".to_string();
        let sanitized = original.sanitize();
        assert_eq!(sanitized, expected);
    }

    #[test]
    fn test_sanitize_with_no_special_chars() {
        let original = "hello world".to_string();
        let expected = original.clone();
        let sanitized = original.sanitize();
        assert_eq!(sanitized, expected);
    }

    #[test]
    fn test_to_message() {
        let defs = vec![Definition {
            word: "test".to_string(),
            meanings: vec![
                Meaning {
                    part_of_speech: "noun".to_string(),
                    definitions: vec![
                        DefinitionDetail {
                            definition: "a procedure intended to establish the quality, performance, or reliability of something, especially before it is taken into widespread use".to_string(),
                            antonyms: Some(vec!["flunk".to_string()]),
                            synonyms: Some(vec!["exam".to_string(), "trial".to_string()]),
                            example: Some("this is only a test".to_string()),
                        },
                    ],
                }
            ],
        }];

        let expected = "*[noun]*\n- a procedure intended to establish the quality, performance, or reliability of something, especially before it is taken into widespread use\n\n".to_string();

        assert_eq!(defs[0].to_message(), expected);
    }

    #[test]
    fn test_telegram_trait_impl_for_vec_definition() {
        let definitions = vec![
            Definition {
                word: "example".to_string(),
                meanings: vec![
                    Meaning {
                        part_of_speech: "noun".to_string(),
                        definitions: vec![
                            DefinitionDetail {
                                definition: "a thing characteristic of its kind or illustrating a general rule".to_string(),
                                antonyms: Some(vec!["flunk".to_string()]),
                                synonyms: Some(vec!["exam".to_string(), "trial".to_string()]),
                                example: Some("this is only a test".to_string()),
                            }
                        ],
                    }
                ],
            },
            Definition {
                word: "test".to_string(),
                meanings: vec![
                    Meaning {
                        part_of_speech: "verb".to_string(),
                        definitions: vec![
                            DefinitionDetail {
                                definition: "take measures to check the quality, performance, or reliability of (something), especially before putting it into widespread use or practice".to_string(),
                                antonyms: Some(vec!["flunk".to_string()]),
                                synonyms: Some(vec!["exam".to_string(), "trial".to_string()]),
                                example: Some("this is only a test".to_string()),
                            },
                        ],
                    }
                ],
            },
        ];

        let expected_output = "\
            *Definitions for* _example_:\n\
            *[noun]*\n- a thing characteristic of its kind or illustrating a general rule\n\
            \n\
            *Definitions for* _test_:\n\
            *[verb]*\n- take measures to check the quality, performance, or reliability of \\(something\\), especially before putting it into widespread use or practice\n\
            \n\
        ";

        assert_eq!(definitions.to_message(), expected_output);
    }
}
