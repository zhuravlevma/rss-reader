use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkDto {
    pub link_id: String,
    pub name: String,
    pub link: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkCreatedDto {
    pub link_id: String,
    pub name: String,
    pub link: String,
}
