use crate::core::db::get_mongo;
use bson::{doc, document::Document, Bson};

use mongodb::{error::Error, options::*, results::*, Client, Collection};

const MONGO_DB: &'static str = "stated-rust";

fn get_collection(client: &Client, collection: &str) -> Collection {
    client.database(MONGO_DB).collection(collection)
}

//insert operation
pub async fn insert(collection: &str, doc: &Document) -> Result<InsertOneResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = get_collection(client, collection);
    collection.insert_one(doc.clone(), None).await
}

pub async fn insert_many(
    collection: &str,
    docs: impl IntoIterator<Item = Document>,
    options: impl Into<Option<InsertManyOptions>>,
) -> Result<InsertManyResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = get_collection(client, collection);
    collection.insert_many(docs, options).await
}

pub async fn find_one(collection: &str, id: String) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(doc! { "id" : id }, None).await
}

pub async fn find_one_and_update(
    collection: &str,
    id: String,
    doc: Document,
    options: Option<FindOneAndUpdateOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let update_doc = update_doc(doc);
    collection
        .find_one_and_update(doc! { "id" : id }, update_doc, options)
        .await
}

pub async fn find_one_and_delete(
    collection: &str,
    id: String,
    options: Option<FindOneAndDeleteOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection
        .find_one_and_delete(doc! { "id" : id }, options)
        .await
}

pub async fn find_one_by(
    collection: &str,
    filter: Option<Document>,
    options: Option<FindOneOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(filter, options).await
}

pub fn update_doc(doc: Document) -> Document {
    doc! {
        "$set": doc
    }
}
