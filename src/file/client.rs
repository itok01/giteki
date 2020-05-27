//use crate::endpoint;
//use crate::file::model::RequestParameters;
//
//// なぜかファイルが見つからない
//// TODO: ↑を直す
//pub async fn get(query: &RequestParameters) -> Result<bytes::Bytes, Box<dyn //std::error::Error>> {
//    let client = reqwest::Client::new();
//
//    let resp = client.get(endpoint::FILE).query(query).send().await?;
//
//    //let status = resp.status();
//
//    //println!("{}", resp.text().await?);
//
//    //Ok(())
//    Ok(resp.bytes().await?)
//}
