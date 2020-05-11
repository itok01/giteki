use crate::endpoint;
use crate::list::model::{RequestParameters, Response};

pub async fn get(query: &RequestParameters) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client.get(endpoint::LIST).query(query).send().await?;

    //let status = resp.status();
    let text: String = resp.text().await?;
    let text = text.replace(r#""giteki":[{"#, r#""giteki":["#);
    let text = text.replace(r#""gitekiInfo":{"#, "{");
    let text = text.replace("}},{{", "},{");
    let text = text.replace("}}]", "}]");
    println!("{}", text);

    let r = serde_json::from_str::<Response>(text.as_str())?;
    println!("{:?}", r.giteki_information.last_update_date);

    Ok(r)
}

#[tokio::test]
async fn get_test() {
    let mut req_params = RequestParameters::new();
    req_params.set_nam("google");
    req_params.set_num("003-180123");
    req_params.set_tn("g013d");
    req_params.set_dc(1);

    let resp: Response = get(&req_params).await.unwrap();

    assert!(resp.giteki_information.last_update_date.is_ascii());
    assert!(resp.giteki_information.total_count > 0);
    assert!(resp.giteki.len() > 0);
}
