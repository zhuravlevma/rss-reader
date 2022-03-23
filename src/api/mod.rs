use crate::dto::{AccessTokenDto, ContentDto, LinkCreatedDto, LinkDto, UserDto};
use js_sys::JSON;
use log::info;
use reqwasm::http::Request;
use reqwasm::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct SignError {
    status: u16,
    message: String,
}

pub async fn sign_in_api(username: String, password: String) -> Result<AccessTokenDto, Error> {
    let body = json!({
        "username": username,
        "password": password,
    });
    let js_body = JsValue::from_serde(&body).unwrap();
    let json = JSON::stringify(&js_body).unwrap();
    let res = Request::post("http://127.0.0.1:3000/login")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;
    info!("{} {}", res.ok(), res.status());
    let res = res.json().await?;
    Ok(res)
}

pub async fn sign_up_api(username: String, password: String) -> Result<UserDto, Error> {
    let body = json!({
        "username": username,
        "password": password,
    });
    let js_body = JsValue::from_serde(&body).unwrap();
    let json = JSON::stringify(&js_body).unwrap();
    let res = Request::post("http://127.0.0.1:3000/user")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;
    info!("{}", res.ok());
    let res = res.json().await?;
    Ok(res)
}

pub async fn get_links(token: String) -> Result<Vec<LinkDto>, Error> {
    let res = Request::get("http://127.0.0.1:3000/link")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn get_content(token: String, start: u32, take: u32) -> Result<Vec<ContentDto>, Error> {
    let res = Request::get(&format!(
        "http://127.0.0.1:3000/content?start={}&take={}",
        start, take
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .unwrap()
    .json()
    .await?;
    Ok(res)
}

pub async fn create_link(token: String, link_url: String) -> Result<LinkCreatedDto, Error> {
    let body = json!({
        "link": link_url,
    });
    let js_body = JsValue::from_serde(&body).unwrap();
    let json = JSON::stringify(&js_body).unwrap();
    let res = Request::post("http://127.0.0.1:3000/link")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(json)
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn remove_link(token: String, link_id: String) -> Result<bool, Error> {
    let res = Request::delete(&format!("http://127.0.0.1:3000/link/{}", link_id,))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}
