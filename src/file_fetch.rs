use std::collections::HashMap;

use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

pub async fn get_data<T: DeserializeOwned>(path: String) -> Result<T, String> {
    let client = reqwest::Client::new();
    let api_call = client.get(path.clone()).send().await;
    match api_call {
        Ok(resp) => {
            let file = resp.json::<T>().await;
            match file {
                Ok(result) => Ok(result),
                Err(err) => Err(String::from(format!("Error parsing get data : {} {}", path, err))),
            }
        }
        Err(e) => Err(String::from(format!("Error fetching get data : {} {}",path, e))),
    }
}

pub async fn post_data<T: DeserializeOwned>(
    path: String,
    body: JsonValue,
    headers: HashMap<String, String>,
) -> Result<T, String> {
    let client = reqwest::Client::new();
    let mut req_headers: HeaderMap = HeaderMap::new();
    if let Some(auth_token) = headers.get("auth_token") {
        req_headers.insert(AUTHORIZATION, auth_token.clone().parse().unwrap());
    }
    let api_call = client
        .post(path.clone())
        .headers(req_headers)
        .json(&body)
        .send()
        .await;
    match api_call {
        Ok(resp) => {
            let file = resp.json::<T>().await;
            match file {
                Ok(result) => Ok(result),
                Err(err) => Err(String::from(format!("Error parsing post data : {} {} {}", path, body, err))),
            }
        }
        Err(e) => Err(String::from(format!("Error fetching post data : {} {} {}",path, body, e))),
    }
}
