use crate::utils::hasher::HASHER;
use bson::{DateTime, Document};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: String,
    pub tittle: String,
    pub description: String,
    pub format: String,
    pub suitability: Vec<String>,
    pub duration: DateTime,
    pub directors: Vec<String>,
    pub language: String,
    pub status: i32,
    pub nation: String,
    pub release: DateTime,
    pub rating: u32,
    pub keyword: Vec<String>,
    pub trailer: Vec<String>,
    pub category: Vec<String>,
    pub ibm: u32,
    pub ticket_sold: i64,
    pub studio: String,
    pub created_by: String,
    pub created_time_dt: DateTime,
    pub updated_by: String,
    pub updated_time_dt: DateTime,
}
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct AddMovie {
    pub tittle: String,
}
