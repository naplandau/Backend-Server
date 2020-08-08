use dotenv::dotenv;
use serde::Deserialize;
#[derive(Clone, Deserialize, Debug)]
pub struct Config{
    //pub auth_salt: String,
    //pub database: DatabaseConnection,
    //pub jwt_expiration: i64,
    //pub jwt_key: String,
    //pub rust_backtrace:u8,
    //pub rust_log:String,
    pub secret_key: String,
    pub server: String,
    //pub session_key: String,
    //pub session_name:String,
    //pub session_secure: bool,
    //pub session_timeout: i64
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}
fn get_config() -> Config{
    dotenv().ok();

    match envy::from_env::<Config>(){
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}",error)
    }
}

