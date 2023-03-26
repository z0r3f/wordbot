use reqwest::Error;

async fn get_definition(word: &str) -> Result<String, Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = reqwest::get(&url).await?.text().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_definition() {
        let word = "Hello";
        let definition = get_definition(word).await.unwrap();
        assert!(definition.contains(word));
    }
}
