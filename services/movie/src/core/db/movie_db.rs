use super::db_utils;
use crate::core::models::Movie;
use crate::utils::hasher::HASHER;
use bson::{doc, Document};
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

impl From<Document> for Movie {
    fn from(doc: Document) -> Self {
        let movie: Movie = bson::from_document(doc).unwrap();
        movie
    }
}

impl From<Movie> for Document {
    fn from(movie: Movie) -> Self {
        bson::to_document(&movie).unwrap()
    }
}
