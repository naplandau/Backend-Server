// use crate::core::db::db_utils;
use crate::utils::handlers::HASHER;
use bson::{doc, DateTime, Document};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub role: String,
    //pub roles: String,
    //pub avatar: String,
    pub created_by: String,
    pub created_time_dt: DateTime,
    pub updated_by: String,
    pub updated_time_dt: DateTime,
    pub status: i32,
}
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserResponse {
//     pub id: String,
//     pub email: String,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub phone_number: Option<String>,
//     pub role: Option<String>,
//     pub roles: Option<String>,
//     pub avatar: Option<String>,
//     pub status: i8,
// }

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Login {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: DateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Register {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Update {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<i8>,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Delete {
    pub email: String,
}
#[allow(dead_code)]
pub struct Auth {
    pub token: String,
}
lazy_static! {
    pub static ref ADMIN_DOC: Document = doc! {
        "id": "user_06e75640-09b0-4b8f-b06f-a4af47aebd4a".to_string(),
        "email": "admin@gmail.com".to_string(),
        "password": HASHER.hash("123456789").unwrap(),
        "first_name": "ADMIN".to_string(),
        "last_name": "".to_string(),
        "phone_number": "+84767336687".to_string(),
        "role": "ADMIN".to_string(),
        //"roles": "ADMIN".to_string(),
        //"avatar": "".to_string(),
        //"time_zone": 7,
        "created_by": "admin".to_string(),
        "created_time_dt": Utc::now(),
        "updated_by": "admin".to_string(),
        "updated_time_dt": Utc::now(),
        "status": 1,
        //"confirm_code": "admin".to_string(),
        //"confirm_code_created_time_dt": Utc::now(),
    };
}
