use dotenv::dotenv;
use serde::Deserialize;
#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    //pub auth_salt: String,
    pub database_url: String,
    //pub jwt_expiration: i64,
    //pub jwt_key: String,
    //pub rust_backtrace:u8,
    //pub rust_log:String,
    pub secret_key: String,
    pub server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: i64,
    pub domain: String,
    //pub session_key: String,
    //pub session_name:String,
    //pub session_secure: bool,
    //pub session_timeout: i64
}
// pub fn get_secret_key() -> String {
//     std::env::var("SECRET_KEY").expect("SECRET_KEY must be set")
// }
// pub fn get_smtp_username() -> String {
//     std::env::var("SMTP_USER_NAME").expect("")
// }
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
