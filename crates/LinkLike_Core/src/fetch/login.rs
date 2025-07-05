use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;
use crate::fetch::header::login_headers;
use crate::url::LOGIN_URL;

// generate a random hex
fn rand_hex_string(len: usize) -> String {
    use rand::{thread_rng, Rng};
    const HEX: &[u8] = b"abcdef0123456789";
    let mut rng = thread_rng();
    (0..len)
        .map(|_| HEX[rng.gen_range(0..HEX.len())] as char)
        .collect()
}

pub async fn login(client_version: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    // body
    let body = json!({
        "device_specific_id": "",
        "player_id": "",
        "version": 1
    });

    // headers
    let mut headers = HeaderMap::new();
    for (k, v) in login_headers() {
        let value = if k == "X-Idempotency-Key" {
            rand_hex_string(32)
        } else if k == "X-Client-Version" {
            client_version.to_string()
        } else if k == "User-Agent" {
            format!("inspix-android/{}", client_version)
        } else {
            v.to_string()
        };
        headers.insert(
            HeaderName::from_bytes(k.as_bytes())?,
            HeaderValue::from_str(&value)?,
        );
    }

    let res = client
        .post(LOGIN_URL)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("Login failed. Status: {}", res.status()).into());
    }

    let res_info = res
        .headers()
        .get("x-res-version")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    Ok(res_info)
}