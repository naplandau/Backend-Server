use crate::core::db::get_mongo;
use bson::{doc, document::Document, Bson};
use mongodb::{error::Error, options::FindOptions, results::*, Client, Collection, Cursor};
const MONGO_DB: &'static str = "stated-rust";

fn get_collection(client: &Client, col_name: &str) -> Collection {
    client.database(MONGO_DB).collection(col_name)
}

pub async fn insert(col_name: &str, doc: &Document) -> Result<InsertOneResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = get_collection(client, col_name);
    collection.insert_one(doc.clone(), None).await
}
pub async fn find(col_name: &str, id: String) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.find_one(doc! {"id": id}, None).await
}
pub async fn find_by(col_name: &str, field: Document) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.find_one(field, None).await
}
pub async fn find_all_with_filter(
    col_name: &str,
    filter: Document,
    find_options: FindOptions,
) -> Result<Cursor, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.find(filter, find_options).await
}

pub async fn find_all(col_name: &str) -> Result<Cursor, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.find(doc! {}, None).await
    //let docs = Vec<_> = .map(|doc| doc.unwrap()).collect)
}

pub async fn update(
    col_name: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.update_one(query.clone(), update, None).await
}

pub async fn update_all(
    col_name: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.update_many(query.clone(), update, None).await
}

pub async fn delete(
    client: &Client,
    col_name: &str,
    query: Document,
) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.delete_one(query.clone(), None).await
}

pub async fn delete_all(
    client: &Client,
    col_name: &str,
    query: Document,
) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.delete_many(query.clone(), None).await
}

pub async fn count_filter(col_name: &str, filter: Document) -> Result<i64, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.count_documents(filter.clone(), None).await
}
pub async fn find_distinct_value(
    col_name: &str,
    field_name: String,
    filter: Document,
) -> Result<Vec<Bson>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection
        .distinct(&*field_name, filter.clone(), None)
        .await
}
pub async fn drop(col_name: &str) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, col_name);
    collection.drop(None).await
}

pub async fn create_collection(col_name: &str) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    client
        .database(MONGO_DB)
        .create_collection(&*col_name, None)
        .await
}
