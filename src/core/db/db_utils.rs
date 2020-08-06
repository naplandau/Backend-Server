use bson::{doc, document::Document, Bson};
use mongodb::{error::Error, options::FindOptions, results::*, Client, Collection, Cursor};
const MONGO_DB: &'static str = "stated-rust";

fn get_collection(client: &Client, col_name: &str) -> Collection {
    client.database(MONGO_DB).collection(col_name)
}

async fn insert(
    client: &Client,
    col_name: &str,
    doc: &Document,
) -> Result<InsertOneResult, Error> {
    let collection = get_collection(client, col_name);
    collection.insert_one(doc.clone(), None).await
}
pub async fn find(client: &Client, col_name: &str) -> Result<Option<Document>, Error> {
    let collection = &get_collection(client, col_name);
    collection.find_one(doc! {}, None).await
}

async fn find_filter(
    client: &Client,
    col_name: &str,
    filter: Document,
    find_options: FindOptions,
) -> Result<Cursor, Error> {
    let collection = &get_collection(client, col_name);
    collection.find(filter, find_options).await
}

pub async fn find_all(client: &Client, col_name: &str) -> Result<Cursor, Error> {
    let collection = &get_collection(client, col_name);
    collection.find(doc! {}, None).await
    //let docs = Vec<_> = .map(|doc| doc.unwrap()).collect)
}

async fn update(
    client: &Client,
    col_name: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.update_one(query.clone(), update, None).await
}

async fn update_all(
    client: &Client,
    col_name: &str,
    query: Document,
    update: Document,
) -> Result<UpdateResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.update_many(query.clone(), update, None).await
}

async fn delete(client: &Client, col_name: &str, query: Document) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.delete_one(query.clone(), None).await
}

async fn delete_all(
    client: &Client,
    col_name: &str,
    query: Document,
) -> Result<DeleteResult, Error> {
    let collection = &get_collection(client, col_name);
    collection.delete_many(query.clone(), None).await
}

async fn count_filter(client: &Client, col_name: &str, filter: Document) -> Result<i64, Error> {
    let collection = &get_collection(client, col_name);
    collection.count_documents(filter.clone(), None).await
}
async fn find_distinct_value(
    client: &Client,
    col_name: &str,
    field_name: String,
    filter: Document,
) -> Result<Vec<Bson>, Error> {
    let collection = &get_collection(client, col_name);
    collection
        .distinct(&*field_name, filter.clone(), None)
        .await
}
async fn drop(client: &Client, col_name: &str) -> Result<(), Error> {
    let collection = &get_collection(client, col_name);
    collection.drop(None).await
}

async fn create_collection(client: &Client, col_name: &str) -> Result<(), Error> {
    client
        .database(MONGO_DB)
        .create_collection(&*col_name, None)
        .await
}