use crate::config::config::CONFIG;
// use actix::prelude::{Actor, SyncContext};
// use actix::prelude::{Addr, SyncArbiter};
// use actix_web::web;
use mongodb::{
    //error::Error, 
    Client};
use once_cell::sync::OnceCell;
// use std::env;
// use std::sync::*;
use tokio;

static MONGO: OnceCell<Client> = OnceCell::new();
static MONGO_INIT: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();
pub async fn get_mongo() -> Option<&'static Client> {
    if let Some(v) = MONGO.get() {
        return Some(v);
    }

    let initializing_mutex = MONGO_INIT.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        let database_url = &CONFIG.database_url;
        if let Ok(client) = Client::with_uri_str(database_url.as_str()).await {
            if let Ok(_) = MONGO.set(client) {
                *initialized = true;
            }
        }
    }
    drop(initialized);
    MONGO.get()
}
