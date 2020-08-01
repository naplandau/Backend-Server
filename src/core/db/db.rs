use actix::prelude::{Actor, SyncContext};
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{self,ConnectionManager, Pool, PooledConnection,PoolError},
};
use actix::prelude::{Addr, SyncArbiter};
use std::env;
use dotenv::dotenv;
pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
//pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

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
    let database_pool = new_pool(database_url).expect("Fail to create pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    let state = AppState {
        db: database_address.clone(),
    };
    state
}
pub fn add_pool(cfg: &mut web::ServiceConfig){
    cfg.data(app_state());
}
pub fn new_pool<S: Into<String>>(database_url: S) -> Result<PgPool,PoolError>{
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    r2d2::Pool::builder().build(manager)
}