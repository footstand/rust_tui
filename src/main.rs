use std::io;
use thiserror::Error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

fn main() {
    println!("Hello, world!");
    println!("DB_PATH: {0}", DB_PATH);
}
