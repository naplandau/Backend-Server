use super::db_utils;
use crate::core::models::Movie;
use crate::utils::hasher::HASHER;
use bson::{Document, document::DocumentIntoIterator, doc};
use chrono::Utc;
use mongodb::{error::Error, options::*};
use std::iter::Iterator;

const COLLECTION_NAME: &str = "movies";

pub async fn insert(movie: Movie) -> Result<String, Error> {
    let ret = db_utils::insert(COLLECTION_NAME, &movie.into()).await;
    match ret {
        Ok(res) => Ok(res.inserted_id.to_string()),
        Err(error) => Err(Error::from(error)),
    }
}

pub async fn find_all() -> Result<Vec<Movie>, Error> {
    let res = db_utils::find_many(COLLECTION_NAME, None, None).await;
    match res {
        Ok(docs) => Ok(docs.into_iter().map(|doc| doc.into()).collect()),
        Err(error) => Err(Error::from(error)),
    }
}

pub async fn find_by_id(id: String) -> Result<Option<Movie>, Error> {
    let res = db_utils::find_one(COLLECTION_NAME, id).await;
    match res {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None),
        },
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn update(movie: Movie, doc: Document) -> Result<Movie, Error> {
    let query = doc! {
        "id": movie.to_owned().id
    };
    let res = db_utils::update_one(COLLECTION_NAME, query, update_time(doc.to_owned()), None).await;
    match res {
        Ok(_) => Ok(handle_update(movie.to_owned(), doc.to_owned())),
        Err(e) => Err(Error::from(e))
    }
}

pub async fn delete_by_id(id: String) -> Result<Option<Movie>, Error> {
    let res = db_utils::find_one_and_delete(COLLECTION_NAME, id, None).await;
    match res {
        Ok(op) => match op {
            Some(doc) => Ok(Some(doc.into())),
            None => Ok(None)
        }
        Err(e) => Err(Error::from(e))
    }
}

impl From<Document> for Movie {
    fn from(doc: Document) -> Self {
        bson::from_document(doc).unwrap()
    }
}

impl From<Movie> for Document {
    fn from(movie: Movie) -> Self {
        bson::to_document(&movie).unwrap()
    }
}

fn update_time(mut update: Document) -> Document {
    update.insert("updated_time_dt", bson::Bson::DateTime(Utc::now()));
    update
}

fn handle_update(movie: Movie, doc: Document) -> Movie {
    let mut movie_doc = bson::to_document(&movie).unwrap();
    movie_doc.extend(doc);
    bson::from_document(movie_doc).unwrap()
}