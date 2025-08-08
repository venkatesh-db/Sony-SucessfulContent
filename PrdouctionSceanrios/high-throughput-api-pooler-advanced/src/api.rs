
use anyhow::Result;
use reqwest;

pub async fn call_api(url: &str) -> Result<String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| anyhow::anyhow!("Request error: {}", e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Response parse error: {}", e))?;

    Ok(response)
}
