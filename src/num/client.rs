use crate::endpoint;
use crate::num::model::{RequestParameters, Response};

pub async fn get(query: &RequestParameters) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client.get(endpoint::NUM).query(query).send().await?;

    //let status = resp.status();
    let text = resp.text().await?;

    Ok(serde_json::from_str(&text)?)
}
