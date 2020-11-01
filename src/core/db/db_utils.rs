use crate::core::db::get_mongo;
use bson::{doc, document::Document, Bson};
use futures::StreamExt;
use mongodb::{error::Error, options::*, results::*, Client, Collection};
// use crate::config::config::CONFIG;
const MONGO_DB: &'static str = "stated-rust";

fn get_collection(client: &Client, collection: &str) -> Collection {
    client.database(MONGO_DB).collection(collection)
}

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

/// `Find document by id`
pub async fn find_one(collection: &str, id: String) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(doc! {"id": id}, None).await
}
pub async fn find_one_and_update(
    collection: &str,
    id: String,
    update: Document,
    options: Option<FindOneAndUpdateOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let update_doc = parse_update_doc(update);
    collection
        .find_one_and_update(doc! {"id": id}, update_doc, options)
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
        .find_one_and_delete(doc! {"id": id}, options)
        .await
}

/// `filter = doc!{"key" : value, "key" : value, ...}`
pub async fn find_one_by(
    collection: &str,
    filter: Option<Document>,
    options: Option<FindOneOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one(filter, options).await
}
pub async fn find_one_by_and_update(
    collection: &str,
    filter: Document,
    update: Document,
    options: Option<FindOneAndUpdateOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let update_doc = parse_update_doc(update);
    collection
        .find_one_and_update(filter, update_doc, options)
        .await
}
pub async fn find_one_by_and_delete(
    collection: &str,
    filter: Document,
    options: Option<FindOneAndDeleteOptions>,
) -> Result<Option<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.find_one_and_delete(filter, options).await
}

pub async fn find_many(
    collection: &str,
    filter: Option<Document>,
    options: Option<FindOptions>,
) -> Result<Vec<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let cursor = collection.find(filter, options).await;
    match cursor {
        Ok(c) => {
            let docs: Vec<_> = c.map(|doc| doc.unwrap()).collect().await;
            Ok(docs)
        }
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn update_one(
    collection: &str,
    query: Document,
    update: Document,
    options: Option<UpdateOptions>,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let update_doc = parse_update_doc(update);
    collection.update_one(query, update_doc, options).await
}

pub async fn update_many(
    collection: &str,
    query: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> Result<UpdateResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.update_many(query, update, options).await
}

pub async fn delete_one(
    collection: &str,
    query: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> Result<DeleteResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.delete_one(query, options).await
}

pub async fn delete_many(
    collection: &str,
    query: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> Result<DeleteResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.delete_many(query, options).await
}

pub async fn delete_all(collection: &str) -> Result<DeleteResult, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.delete_many(doc! {}, None).await
}

pub async fn count(
    collection: &str,
    filter: Option<Document>,
    options: Option<CountOptions>,
) -> Result<i64, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.count_documents(filter, options).await
}
pub async fn estimate_count(
    collection: &str,
    options: impl Into<Option<EstimatedDocumentCountOptions>>,
) -> Result<i64, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.estimated_document_count(options).await
}

pub async fn find_distinct_value(
    collection: &str,
    field_name: String,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<DistinctOptions>>,
) -> Result<Vec<Bson>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.distinct(&*field_name, filter, options).await
}

pub async fn aggregate(
    collection: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
) -> Result<Vec<Document>, Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    let cursor = collection.aggregate(pipeline, options).await;
    match cursor {
        Ok(c) => {
            let docs: Vec<_> = c.map(|doc| doc.unwrap()).collect().await;
            Ok(docs)
        }
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn drop_collection(
    collection: &str,
    options: Option<DropCollectionOptions>,
) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    let collection = &get_collection(client, collection);
    collection.drop(options).await
}
pub async fn create_collection(
    collection: &str,
    options: Option<CreateCollectionOptions>,
) -> Result<(), Error> {
    let client = get_mongo().await.unwrap();
    client
        .database(MONGO_DB)
        .create_collection(&*collection, options)
        .await
}

fn parse_update_doc(doc: Document) -> Document {
    doc! {
        "$set": doc
    }
}
