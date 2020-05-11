use crate::endpoint;
use crate::num::model::{RequestParameters, Response};

pub async fn get(query: &RequestParameters) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client.get(endpoint::NUM).query(query).send().await?;

    //let status = resp.status();
    let text = resp.text().await?;

    Ok(serde_json::from_str(&text)?)
}

#[tokio::test]
async fn get_test() {
    let mut req_params = RequestParameters::new();
    req_params.set_tec("1");

    let resp: Response = get(&req_params).await.unwrap();

    assert!(resp.giteki_information.last_update_date.is_ascii());
    assert!(resp.giteki_information.total_count > 0);
    assert!(resp.giteki.count > 0);
}
