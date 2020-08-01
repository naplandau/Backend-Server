use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::{Queryable, Identifiable, Insertable};
use super::schema::users;

#[derive(Debug,Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub dob: NaiveDateTime,
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

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser{
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub dob: NaiveDateTime,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub status: i16,
    pub confirm_code: Option<String>,
    pub confirm_code_created_time_dt: NaiveDateTime
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub dob: NaiveDateTime,
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