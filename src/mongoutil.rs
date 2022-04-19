use mongodb::{bson, Collection};
use bson::Document;
use serde::{Serialize, de::DeserializeOwned};

use std::marker::{Unpin, Send};

pub fn bson_to_docs<T>(x:&T) -> Document 
    where T: Sized + Serialize {
    let bson = bson::to_bson(x).unwrap();
    return bson::from_bson::<Document>(bson).unwrap();
}

pub async fn is_exist<T>(collection: Collection<T>, docs: Document) -> (Collection<T>, Document, bool) 
    where T: DeserializeOwned + Unpin + Send + Sync 
{
    return (collection.clone(), docs.clone(), match collection.find_one(docs, None).await.unwrap_or(None) {
        Some(_) => { true },
        None => { false }
    });
}