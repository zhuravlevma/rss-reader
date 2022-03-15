use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkDto {
    pub link_id: String,
    pub name: String,
    pub link: String,
    pub description: String,
}