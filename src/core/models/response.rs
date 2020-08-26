use bson::Document;
// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub data: String,
    pub status: bool,
    pub request_id: String,
    pub auth_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Document,
    pub message: String,
    pub status: bool,
}
