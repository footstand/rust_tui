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

fn main() {
    println!("Hello, world!");
    println!("DB_PATH: {0}", DB_PATH);
}
