#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate env_logger;

mod server;
mod app;
mod config;
mod core;
use crate::server::server::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    server().await
}
