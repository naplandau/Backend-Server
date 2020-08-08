use actix::prelude::{Actor, SyncContext};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;
use mongodb::{error::Error, Client};
use std::env;
use std::sync::*;
use once_cell::sync::OnceCell;
//use bson::doc;
use tokio;

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
#[allow(dead_code)]
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor(pub PgPool);
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
pub struct AppState {
    pub db: Addr<DbExecutor>,
}
pub fn app_state() -> AppState {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = init_pgpool(database_url).expect("Failed to create pool");
    let database_address =
        SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    let state = AppState {
        db: database_address.clone(),
    };
    state
}
pub fn add_pool(cfg: &mut web::ServiceConfig) {
    cfg.data(app_state());
}
pub fn init_pgpool<S: Into<String>>(database_url: S) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    r2d2::Pool::builder().build(manager)
}
pub async fn init_mongopool<S: Into<String>>(
    database_url: S,
) -> Result<web::Data<Mutex<Client>>, Error> {
    // let mut client_options = ClientOptions::parse(&*database_url.into()).await.unwrap();
    // client_options.app_name = Some("Started-Rust".to_string());
    // let client = Client::with_options(client_options).unwrap();
    let client = Client::with_uri_str(&*database_url.into()).await?;
    Ok(web::Data::new(Mutex::new(client)))
}
static MONGO: OnceCell<Client> = OnceCell::new();
static MONGO_INIT: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();
pub async fn get_mongo() -> Option<&'static Client>{
    if let Some(v) = MONGO.get(){
        return Some(v);
    }

    let initializing_mutex = MONGO_INIT.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    if !*initialized{
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        if let Ok(client) = Client::with_uri_str(&*database_url).await{
            if let Ok(_) = MONGO.set(client){
                *initialized = true;
            }
        }
    }
    drop(initialized);
    MONGO.get()
}