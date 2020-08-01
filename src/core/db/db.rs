use crate::config::config::{Config, CONFIG};
use actix_web::web;
use actix::prelude;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection,PoolError},
    Connection
}
use std::env;
pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub fn init() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Fail to create pool.");
    

}
pub fn new_pool<S: Into<String>>(database_url: S) ->Result<PgPool>{
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(poll)
}