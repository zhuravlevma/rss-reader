use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserDto {
    pub user_id: String,
    pub username: String,
    pub password: String,
}
