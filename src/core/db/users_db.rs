use super::db_utils;
//use crate::config::config::CONFIG;
use crate::core::models::{
    // response::*, 
    users::User,
users::Confirmation};
// use crate::utils::handlers::hasher::{hash_validation,     HASHER};
use bson::doc;
// use bson::{Bson, Document};
// use chrono::{DateTime, Duration, Utc};
// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{error::Error, 
    // options::FindOptions
};
// use uuid::Uuid;

const COLLECTION_NAME: &str = "users";
const PENDING_COLLECTION: &str = "users_pending";

pub async fn find(id: String) -> Result<Option<User>, Error> {
    let doc = db_utils::find(COLLECTION_NAME, id).await.unwrap();
    match doc {
        Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        },
        None => Ok(None),
    }
}
pub async fn find_by_email(email: String) -> Result<Option<User>, Error> {
    let field = doc! {
        "email": email
    };
    
    let doc = db_utils::find_by(COLLECTION_NAME, field);
    Ok(None)
    // match doc {
    //     Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
    //         Ok(model) => Ok(model),
    //         Err(e) => Err(Error::from(e)),
    //     },
    //     None => Ok(None),
    // }
}
pub async fn find_pending(id: String) -> Result<Option<Confirmation>, Error> {
    let field = doc! {
        "id": id
    };
    
    let doc = db_utils::find_by(PENDING_COLLECTION, field);
    Ok(None)
    // match doc {
    //     Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
    //         Ok(model) => Ok(model),
    //         Err(e) => Err(Error::from(e)),
    //     },
    //     None => Ok(None),
    // }
}
// pub async fn find_all(filter: Document, option: FindOptions) -> Result<Option<User>, Error> {
//     let cursor = db_utils::find_all_with_filter(COLLECTION_NAME, filter, option).await;
//     match cursor {
//         Ok(cursor) => match bson::from_bson(bson::Bson::Document(doc)) {
//             Ok(model) => Ok(model),
//             Err(e) => Err(Error::from(e)),
//         },
//         Error => Ok(None),
//     }
// }
