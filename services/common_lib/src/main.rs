mod database;
mod queue;
// mod services;

#[actix_rt::main]
async fn main(){
    use database::redisdb::*;
    use redis::AsyncCommands;
    let data = r#"{
    "host": "127.0.0.1",
    "port": 27017
    }"#;
    // let res = client.conn.set("","").await;

}