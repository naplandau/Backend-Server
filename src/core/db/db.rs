use actix::prelude::{Actor, SyncContext};
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{self,ConnectionManager, Pool, PooledConnection,PoolError},
};
use actix::prelude::{Addr, SyncArbiter};
use std::env;
use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions, error::Error};
use std::sync::*;
use bson::doc;

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
#[allow(dead_code)]
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor (pub PgPool);
impl Actor for DbExecutor{
    type Context = SyncContext<Self>;
}
pub struct AppState {
    pub db: Addr<DbExecutor>,
}
pub fn app_state() -> AppState{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = init_pgpool(database_url).expect("Failed to create pool");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    let state = AppState {
        db: database_address.clone(),
    };
    state
}
pub fn add_pool(cfg: &mut web::ServiceConfig){
    cfg.data(app_state());
}
pub fn init_pgpool<S: Into<String>>(database_url: S) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    r2d2::Pool::builder().build(manager)
}
pub async fn init_mongopool<S: Into<String>>(database_url: S) ->  Result<web::Data<Mutex<Client>>, Error>{
        // let mut client_options = ClientOptions::parse(&*database_url.into()).await.unwrap();
        // client_options.app_name = Some("Started-Rust".to_string());
        // let client = Client::with_options(client_options).unwrap();
        let client = Client::with_uri_str(&*database_url.into()).await?;
        Ok(web::Data::new(Mutex::new(client)))
    }
    