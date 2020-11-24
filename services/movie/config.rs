use serde::Deserialize;

pub struct Config {
    pub database_url: String,
}

fn get_config() -> Config {
    dotenv::dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(err) => panic!("Configuration Error: {:#?}", error)
    }
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}