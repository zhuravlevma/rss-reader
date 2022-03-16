use crate::dto::{AccessTokenDto, ContentDto, LinkDto, UserDto};
use js_sys::JSON;
use reqwasm::http::Request;
use serde_json::json;
use std::error::Error;
use wasm_bindgen::JsValue;

pub async fn sign_in_api(
    username: String,
    password: String,
) -> Result<AccessTokenDto, Box<dyn Error>> {
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
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn sign_up_api(username: String, password: String) -> Result<UserDto, Box<dyn Error>> {
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
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn get_links(token: String) -> Result<Vec<LinkDto>, Box<dyn Error>> {
    let res = Request::get("http://127.0.0.1:3000/link")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn get_content(
    token: String,
    start: u32,
    take: u32,
) -> Result<Vec<ContentDto>, Box<dyn Error>> {
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
