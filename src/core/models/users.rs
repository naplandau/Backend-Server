use serde::{Deserialize, Serialize};
//use chrono::{NaiveDateTime, Utc};
use bson::{DateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub id: String,
    //pub user_name: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<String>,
    pub created_by: Option<String>,
    pub created_time_dt: DateTime,
    pub updated_by: Option<String>,
    pub updated_time_dt: DateTime,
    pub status: i16,
    pub confirm_code: Option<String>,
    pub confirm_code_created_time_dt: DateTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login{
    //pub user_name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub expire_time: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Register{
    //pub user_name: String,
    pub email: String,
    pub password: String
}