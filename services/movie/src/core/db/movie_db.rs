use super::db_utils;
use bson::{doc, document::Document, Bson};
use futures::StreamExt;
use mongodb::{error::Error, options::*, results::*, Client, Collection};

const COLLECTION_NAME: &str = "movies";

pub async fn insert(movie: Movie) -> Result<String, Error> {
    let ret = db_utils::insert(COLLECTION_NAME, &movie.into()).await;
    match ret {
        Ok(res) => Ok(res.insert_id.to_string()),
        Err(error) => Err(Error::from(error)),
    }
}
