use super::db_utils;
pub use futures::StreamExt;
use crate::core::models::{users::Confirmation, users::User, Update};
use bson::doc;
use bson::{Bson, Document};
use chrono::Utc;
use mongodb::{error::Error, options::FindOptions};
// use uuid::Uuid;

const COLLECTION_NAME: &str = "users";
const PENDING_COLLECTION: &str = "users_pending";
pub async fn insert(user: User) -> Result<String, Error> {
    let docs = prepare_user(user);
    let ret = db_utils::insert(COLLECTION_NAME, &docs).await;
    match ret {
        Ok(res) => Ok(res.inserted_id.to_string()),
        Err(e) => Err(Error::from(e)),
    }
}
// pub async fn update(user: User) -> Result<String, Error> {
//     let docs = prepare_user(user);
//     doc! {"email": doc.email}
//     let ret = db_utils::update(COLLECTION_NAME, &docs).await;
//     match ret {
//         Ok(res) => Ok(res.inserted_id.to_string()),
//         Err(e) => Err(Error::from(e)),
//     }
// }
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

    let doc = db_utils::find_by(COLLECTION_NAME, field).await.unwrap();
    // Ok(None)
    match doc {
        Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        },
        None => Ok(None),
    }
}
pub async fn find_pending(id: String) -> Result<Option<Confirmation>, Error> {
    let field = doc! {
        "id": id
    };

    let doc = db_utils::find_by(PENDING_COLLECTION, field).await.unwrap();
    // Ok(None)
    match doc {
        Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        },
        None => Ok(None),
    }
}
pub async fn find_all(filter: Document, option: FindOptions) -> Result<Option<Vec<User>>, Error> {
    let cursor = db_utils::find_all_with_filter(COLLECTION_NAME, filter, option).await;
    match cursor {
        Ok(mut cursor) => {
            let mut res: Vec<User> = Vec::new();
            while let Some(result) = cursor.next().await {
                let doc = result.unwrap().clone();
                // match bson::from_bson(bson::Bson::Document(doc)) {
                //     Ok(model) => {
                        res.push(doc_to_user(doc));
                    // }
                    // _ => unimplemented!(),
                // }
            }
            Ok(Some(res))
        }
        Err(e) => Err(Error::from(e)),
    }
}
fn doc_to_user(doc: Document) -> User {
    let user: User = bson::from_document(doc).unwrap();
    user
}
fn user_to_doc(user: User) -> Document {
    let doc: Document = bson::to_document(&user).unwrap();
    doc
}
fn prepare_user(user: User) -> Document {
    let current_time = Utc::now();
    doc! {
        "id": user.id.to_string(),
        "email": user.email.to_string(),
        "password": user.password,
        "first_name": user.first_name.to_string(),
        "last_name": user.last_name.to_string(),
        "phone_number": user.phone_number.to_string(),
        "role": user.role.to_string(),
        "created_by": user.created_by.to_string(),
        "created_time_dt": Bson::DateTime(current_time),
        "updated_by": user.updated_by.to_string(),
        "updated_time_dt": Bson::DateTime(current_time),
        "status": user.status,
    }
}
fn prepare_update(user: User, update_user: Update) -> Document {
    let current_time = Utc::now();
    let mut docs = prepare_user(user);
    let update_doc = doc! {
        "updated_by": update_user.email.to_string(),
        "updated_time_dt": current_time.naive_utc().to_string(),
    };
    docs.extend(update_doc);
    docs
}
