use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
