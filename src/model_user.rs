use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,

    pub login: String,
    pub password: String,

    //pub full_name:String,
    //pub age: i128,

}