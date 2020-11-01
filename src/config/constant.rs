// use serde::Deserialize;
// #[derive(Clone, Deserialize, Debug)]
// pub struct ConstValue {

// }

// lazy_static! {
//     pub static ref CONST_VARIABLE: ConstValue = get_const();
// }
// fn get_const() -> Config {
//     dotenv::dotenv().ok();

//     match envy::from_env::<ConstValue>() {
//         Ok(config) => config,
//         Err(error) => panic!("Configuration Variable Error: {:#?}", error),
//     }
// }
