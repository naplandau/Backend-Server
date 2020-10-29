use super::db_utils;
use crate::core::models::{Confirmation, Update, User};
use crate::utils::handlers::HASHER;
use bson::{doc, Bson, Document};
use chrono::Utc;
// use futures::StreamExt;
use std::iter::Iterator;

use mongodb::{error::Error, options::*};

const COLLECTION_NAME: &str = "users";
const PENDING_COLLECTION: &str = "users_pending";

pub async fn insert(user: User) -> Result<String, Error> {
    let ret = db_utils::insert(COLLECTION_NAME, &user.into()).await;
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
pub async fn delete_by_id(id: String) -> Result<Option<User>, Error> {
    let res = db_utils::find_one_and_delete(COLLECTION_NAME, id, None).await;
    match res {
        Ok(op) => {
            match op {
                Some(doc) =>{
                    let user: User = bson::from_document(doc).unwrap();
                    Ok(Some(user))
                },
                None => Ok(None)
            }
        },
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn find_by_id(id: String) -> Result<Option<User>, Error> {
    let doc = db_utils::find_one(COLLECTION_NAME, id).await.unwrap();
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
    let options = FindOneOptions::default();
    let doc = db_utils::find_one_by(COLLECTION_NAME, Some(field), Some(options))
        .await
        .unwrap();
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

    let doc = db_utils::find_one_by(PENDING_COLLECTION, Some(field), None)
        .await
        .unwrap();
    // Ok(None)
    match doc {
        Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        },
        None => Ok(None),
    }
}
pub async fn find_all(filter: Document, option: FindOptions) -> Result<Vec<User>, Error> {
    let docs = db_utils::find_many(COLLECTION_NAME, Some(filter), Some(option)).await;
    match docs {
        Ok(docs_vec) => {
            let res: Vec<User> = docs_vec
                .into_iter()
                .map(|doc| bson::from_document(doc).unwrap())
                .collect();
            Ok(res)
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
impl From<User> for Document {
    fn from(user: User) -> Self {
        let current_time = Utc::now();
        doc! {
            "id": user.id.to_string(),
            "email": user.email.to_string(),
            "password": HASHER.hash(&user.password).unwrap(),
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
}
// fn prepare_update(user: User, update_user: Update) -> Document {
//     let current_time = Utc::now();
//     let mut docs = prepare_user(user);
//     let update_doc = doc! {
//         "updated_by": update_user.email.to_string(),
//         "updated_time_dt": current_time.naive_utc().to_string(),
//     };
//     docs.extend(update_doc);
//     docs
// }
