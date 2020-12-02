#[derive(Serialize, Deserialize, Debug, Clone)]
struct Token{
    access_token: String,
    refresh_token: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenResult{
    user: String,
    access_token: String,
    refresh_token: String,
    signature: String,
    roles: Vec<String>,
    permissions: Vec<String>,
    resources: Vec<String>,
    scopes: Vec<String>,
    status: bool,
    result_code: i64,
    result_description: String,
    auth_time: i64,
}