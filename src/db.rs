use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::models::Test;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub type Database = Arc<Mutex<HashMap<i32, Test>>>;

pub fn init_db() -> Database {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}