use js_sys::JSON;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt::Debug;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String,
}

pub async fn get_users() -> Result<Vec<User>, Box<dyn Error>> {
    let res = Request::get("http://127.0.0.1:3000/user")
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

pub async fn get_user(id: &str) -> Result<User, Box<dyn Error>> {
    let res = Request::get(&format!("http://127.0.0.1:3000/user/{}", id))
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(res)
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AccessToken {
    pub access_token: String,
}
#[derive(Serialize, Deserialize)]
pub struct Body {
    pub username: String,
    pub password: String,
}

pub async fn sign_in_api(
    username: String,
    password: String,
) -> Result<AccessToken, Box<dyn Error>> {
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

pub async fn sign_up_api(username: String, password: String) -> Result<User, Box<dyn Error>> {
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
