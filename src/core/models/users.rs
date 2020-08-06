use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::{Queryable, Identifiable, Insertable};
use super::schema::users;

#[derive(Debug,Queryable, Identifiable, Insertable, Clone, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<String>,
    pub created_by: Option<String>,
    pub created_time_dt: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_time_dt: NaiveDateTime,
    pub status: i16,
    pub confirm_code: Option<String>,
    pub confirm_code_created_time_dt: NaiveDateTime
}

#[derive(Debug, Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser{
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}