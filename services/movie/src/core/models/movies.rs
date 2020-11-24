use crate::utils::hasher::HASHER;
use bson::{DateTime, Document};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: String,
    pub tittle: String,
    pub description: String,
    pub format: String,
    pub suitability: Vec<String>,
    pub duration: DataTime,
    pub directors: Vec<String>,
    pub language: String,
    pub status: i32,
    pub nation: String,
    pub release: DateTime,
    pub rating: u32,
    pub keyword: Vec<String>,
    pub trailer: Vec<String>,
    pub category: Vec<String>,
    pub IBM: u32,
    pub ticketSold: i64,
    pub studio: String,
    pub created_by: String,
    pub created_time_dt: DateTime,
    pub updated_by: String,
    pub updated_time_dt: DateTime,
}

pub struct AddMovie {
    pub tittle: String,
}
