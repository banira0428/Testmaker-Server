use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::User;

pub type Database = Arc<Mutex<HashMap<u64, Test>>>;

pub fn init_db() -> Database {
    Arc::new(Mutex::new(HashMap::new()))
}