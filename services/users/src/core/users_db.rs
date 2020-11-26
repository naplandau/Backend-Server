use super::db_utils;
use crate::models::{Confirmation, User};
use crate::utils::hasher::HASHER;
use bson::{doc, Document};
use chrono::Utc;
use mongodb::{error::Error, options::*};
use std::iter::Iterator;

const COLLECTION_NAME: &str = "users";
const PENDING_COLLECTION: &str = "users_pending";

pub async fn insert(user: User) -> Result<String, Error> {
    let ret = db_utils::insert(COLLECTION_NAME, &user.into()).await;
    match ret {
        Ok(res) => Ok(res.inserted_id.to_string()),
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn update(user: User, update: Document) -> Result<User, Error> {
    let query = doc! {
        "id": user.to_owned().id
    };
    let res = db_utils::update_one(
        COLLECTION_NAME,
        query,
        update_build(update.to_owned()),
        None,
    )
    .await;
    match res {
        Ok(_) => Ok(build_updated_user(user.to_owned(), update.to_owned())),
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn delete_by_id(id: String) -> Result<Option<User>, Error> {
    let res = db_utils::find_one_and_delete(COLLECTION_NAME, id, None).await;
    match res {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn delete_all() -> Result<i64, Error> {
    let res = db_utils::delete_many(COLLECTION_NAME, doc! {}, None).await;
    match res {
        Ok(result) => Ok(result.deleted_count),
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn find_by_id_and_update(user: User, update: Document) -> Result<Option<User>, Error> {
    let res = db_utils::find_one_and_update(
        COLLECTION_NAME,
        user.to_owned().id,
        update_build(update.to_owned()),
        None,
    )
    .await;
    match res {
        Ok(res) => match res {
            Some(_) => Ok(Some(build_updated_user(user.to_owned(), update.to_owned()))),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn find_by_id_and_delete(id: String) -> Result<Option<User>, Error> {
    let result = db_utils::find_one_and_delete(COLLECTION_NAME, id, None).await;
    match result {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn find_by_id(id: String) -> Result<Option<User>, Error> {
    let res = db_utils::find_one(COLLECTION_NAME, id).await;
    match res {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}
pub async fn find_by_email(email: String) -> Result<Option<User>, Error> {
    let filter = Some(doc! {
        "email": email
    });
    let options = FindOneOptions::default();
    let res = db_utils::find_one_by(COLLECTION_NAME, filter, Some(options)).await;
    match res {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}
/// this function is maintaining
pub async fn find_pending(id: String) -> Result<Option<Confirmation>, Error> {
    let filter = Some(doc! {
        "id": id
    });

    let doc = db_utils::find_one_by(PENDING_COLLECTION, filter, None)
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
pub async fn find_all(
    filter: Option<Document>,
    option: Option<FindOptions>,
) -> Result<Vec<User>, Error> {
    let res = db_utils::find_many(COLLECTION_NAME, filter, option).await;
    match res {
        Ok(docs_vec) => {
            let res: Vec<User> = docs_vec.into_iter().map(|doc| doc.into()).collect();
            Ok(res)
        }
        Err(e) => Err(Error::from(e)),
    }
}

fn build_updated_user(user: User, update: Document) -> User {
    let mut user_doc = bson::to_document(&user).unwrap();
    user_doc.extend(update);
    println!("{:#?}", user_doc);
    let user: User = bson::from_document(user_doc).unwrap();
    user
}
fn update_build(mut update: Document) -> Document {
    update.insert("updated_time_dt", bson::Bson::DateTime(Utc::now()));
    update
}
impl From<Document> for User {
    fn from(doc: Document) -> Self {
        let user: User = bson::from_document(doc).unwrap();
        user
    }
}
impl From<User> for Document {
    fn from(mut user: User) -> Self {
        user.password = HASHER.hash(&user.password).unwrap();
        bson::to_document(&user).unwrap()
    }
}
