use crate::url::VERSION_URL;
use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;

// game_id: com.oddno.lovelive

pub async fn get_play_version(game_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", VERSION_URL, game_id);
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 Edg/119.0.0.0")
        .build()?;
    let res = client.get(&url).send().await?;
    if !res.status().is_success() {
        return Err(format!("Abnormal HTTP status code: {}", res.status()).into());
    }
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("script").unwrap();
    let re = Regex::new(r#"\[\[\["([\d\.]+)"\]\],\[\[\[\d+\]\],\[\[\[\d+,"#).unwrap();

    for element in document.select(&selector) {
        let script_content = element.text().collect::<Vec<_>>().join("");
        if script_content.contains("key: 'ds:5'") {
            if let Some(caps) = re.captures(&script_content) {
                return Ok(caps[1].to_string());
            }
        }
    }
    Err("Cannot find ds:5 in <script>, perhaps GooglePlay has changed its structure of webpage.".into())
}