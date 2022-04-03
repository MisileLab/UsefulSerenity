use mongodb::bson;
use bson::Document;
use serde::Serialize;

pub fn bson_to_docs<T>(x:&T) -> Document 
    where T: Sized + Serialize {
    let bson = bson::to_bson(x).unwrap();
    return bson::from_bson::<Document>(bson).unwrap();
}