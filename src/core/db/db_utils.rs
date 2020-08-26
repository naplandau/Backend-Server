use crate::core::db::get_mongo;
use bson::{doc, document::Document, Bson};
use mongodb::{error::Error, options::FindOptions, results::*, Client, Collection, Cursor};

const MONGO_DB: &'static str = "stated-rust";

fn get_collection(client: &Client, collection: &str) -> Collection {
    client.database(MONGO_DB).collection(collection)
}

pub async fn insert(collection: &str, doc: &Document) -> Result<InsertOneResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = get_collection(client, collection);
    collection.insert_one(doc.clone(), None).await
}
pub async fn find(collection: &str, id: String) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(doc! {"id": id}, None).await
}
pub async fn find_by(collection: &str, field: Document) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(field, None).await
}
pub async fn find_all_with_filter(
    collection: &str,
    filter: Document,
    find_options: FindOptions,
) -> Result<Cursor, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find(filter, find_options).await
}

pub async fn find_all(collection: &str) -> Result<Cursor, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find(doc! {}, None).await
    //let docs = Vec<_> = .map(|doc| doc.unwrap()).collect)
}

pub async fn update(
    collection: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.update_one(query.clone(), update, None).await
}

pub async fn update_all(
    collection: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.update_many(query.clone(), update, None).await
}

pub async fn delete(
    client: &Client,
    collection: &str,
    query: Document,
) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, collection);
    collection.delete_one(query.clone(), None).await
}

pub async fn delete_all(
    client: &Client,
    collection: &str,
    query: Document,
) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, collection);
    collection.delete_many(query.clone(), None).await
}

pub async fn count_filter(collection: &str, filter: Document) -> Result<i64, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.count_documents(filter.clone(), None).await
}
pub async fn find_distinct_value(
    collection: &str,
    field_name: String,
    filter: Document,
) -> Result<Vec<Bson>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection
        .distinct(&*field_name, filter.clone(), None)
        .await
}
pub async fn drop(collection: &str) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.drop(None).await
}

pub async fn create_collection(collection: &str) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    client
        .database(MONGO_DB)
        .create_collection(&*collection, None)
        .await
}
