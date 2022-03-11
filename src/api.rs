use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use web_sys::RequestMode;

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
    println!("{:?}", res);
    Ok(res)
}
