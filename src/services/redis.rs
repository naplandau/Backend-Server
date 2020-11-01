// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use redis::{Client, aio::MultiplexedConnection};
// use actix::prelude::*;

// struct RedisActor {
//     conn: MultiplexedConnection,
// }

// impl RedisActor {
//     pub async fn new(redis_url: &'static str) -> Self {
//         let client = Client::open(redis_url).unwrap();// not recommended
//         let (conn, call) = client.get_multiplexed_async_std_connection().await.unwrap();
//         actix_rt::spawn(call);
//         RedisActor { conn }
//     }
// }
//extern crate redis;
// use redis::Commands;

// fn fetch_an_integer() -> redis::RedisResult<isize> {
//     let client = redis::Client::open("redis://127.0.0.1/")?;
//     let mut con = client.get_connection()?;
//     let _ : () = con.set("my_key", 42)?;
//     con.get("my_key")
// }
// #[derive(Message, Debug)]
// #[rtype(result = "Result<Option<String>, redis::RedisError>")]
// struct InfoCommand;

// impl Handler<InfoCommand> for RedisActor {
//     type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

//     fn handle(&mut self, _msg: InfoCommand, _: &mut Self::Context) -> Self::Result {
//         let mut con = self.conn.clone();
//         let cmd = redis::cmd("INFO");
//         let fut = async move {
//             cmd
//                 .query_async(&mut con)
//                 .await
//         };
//         Box::pin(fut)
//     }
// }

// impl Actor for RedisActor {
//     type Context = Context<Self>;
// }
