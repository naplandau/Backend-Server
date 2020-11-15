mod database;
use std::env;
use std::error::Error;
#[actix_rt::main]
async fn main(){
    use database::mongodb::MongoDatabaseFactory;
    let data = r#"{
    "host": "127.0.0.1",
    "port": 27017,
    "app_name": "String",
    "connect_timeout": {
        "secs" :10,
        "nanos" :0
    },
    "credential": null,
    "max_pool_size": null,
    "min_pool_size": null,
    "retry_reads": null,
    "retry_writes": null
    }"#;
    let db = MongoDatabaseFactory::with_config(data).unwrap();
    for name in db.list_database_names(None, None).await.unwrap() {
      println!("- {}", name);
   }
}