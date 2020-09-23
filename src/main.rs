use std::error::Error;

use bson::Bson;
use mongodb::{bson, bson::doc, bson::oid::ObjectId, sync::Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    status: Status,
}
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    NEW,
    OK,
    ERROR,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::NEW => {String::from("NEW")}
            Status::OK => {String::from("OK")}
            Status::ERROR => {String::from("ERROR")}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("test");
    let col = db.collection("test__rust_mongodb_enum");

    // insert data
    let new_data = Data {
        id: None,
        name: String::from("n1"),
        status: Status::OK,
    };
    col.insert_one(bson::to_document(&new_data)?, None)?;

    // find
    let res = col.find_one(doc! {"status": Bson::String(Status::OK.to_string())}, None)?;
    println!("{:#?}", res);

    Ok(())
}
