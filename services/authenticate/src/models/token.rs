#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Token{
    pub access_token: String,
    pub refresh_token: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResult{
    pub user: String,
    pub token: String,
    pub signature: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub resources: Vec<String>,
    pub scopes: Vec<String>,
    pub status: bool,
    pub result_code: i64,
    pub result_description: String,
    pub created_time: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResult{
    pub user: String,
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub status: bool,
    pub result_code: i64,
    pub result_description: String,
    pub created_time: i64
}