use js_sys::JSON;
use log::info;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::JsValue;
use web_sys::{Headers, RequestMode};

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

pub async fn login(username: String, password: String) -> Result<AccessToken, Box<dyn Error>> {
    let body = json!({
        "username": "Ivan",
        "password": "Zhopa",
    });
    // let body = Body { username, password };
    let js_body = JsValue::from_serde(&body).unwrap();
    let json = JSON::stringify(&js_body).unwrap();

    // let js_body_str  = JsValue::into_serde(JsValue::into_serde(&js_body).unwrap())?;

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
