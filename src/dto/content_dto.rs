use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ContentDto {
    pub content_id: String,
    pub link_url: String,
    pub title: String,
    pub description: Option<String>,
    pub date: String,
    pub link_id: String,
    pub logo_url: Option<String>,
}