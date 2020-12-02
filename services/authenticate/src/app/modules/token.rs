use crate::app::lib::*;

pub async fn gen_token()-> String{
    String::from("")
}
pub async fn gen_ref_token() -> String{
    String::from("")
}
pub async fn check_token() -> TokenResult{
    TokenResult{
        user: "".to_owned(),
        access_token: "".to_owned(),
        refresh_token: "".to_owned(),
        signature: "".to_owned(),
        roles: vec![],
        permissions: vec![],
        resources: vec![],
        scopes: vec![],
        status: true,
        result_code: 0,
        result_description: "".to_owned(),
        created_time: 0,
    }
}
pub async fn check_ref_token() -> TokenResult{
    TokenResult{
        user: "".to_owned(),
        access_token: "".to_owned(),
        refresh_token: "".to_owned(),
        signature: "".to_owned(),
        roles: vec![],
        permissions: vec![],
        resources: vec![],
        scopes: vec![],
        status: true,
        result_code: 0,
        result_description: "".to_owned(),
        created_time: 0,
    }
}
pub async fn get_permission() -> Vec<String>{
    vec![]
}
pub async fn check_blacklist() -> AuthResult{
    AuthResult{}
}
