use crate::utils::hasher::HASHER;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: String,
    pub tittle: String,
    pub description: String,
    pub format: String,
    pub suitability: Vec<String>,
    pub duration: i64,
    pub directors: Vec<String>,
    pub language: String,
    pub status: i32,
    pub nation: String,
    pub release: i64,
    pub rating: Option<f32>,
    pub keyword: Vec<String>,
    pub trailer: Vec<String>,
    pub category: Vec<String>,
    pub ibm: Option<f32>,
    pub ticket_sold: Option<i64>,
    pub studio: String,
    pub created_by: String,
    pub created_time_dt: i64,
    pub updated_by: String,
    pub updated_time_dt: i64,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct AddMovie {
    pub tittle: String,
    pub description: String,
    pub format: String,
    pub suitability: Vec<String>,
    pub duration: i64,
    pub directors: Vec<String>,
    pub language: String,
    pub status: i32,
    pub nation: String,
    pub release: i64,
    pub rating: Option<f32>,
    pub keyword: Vec<String>,
    pub trailer: Vec<String>,
    pub category: Vec<String>,
    pub ibm: Option<f32>,
    pub ticket_sold: Option<i64>,
    pub studio: String,
}
