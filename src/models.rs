use serde::{Serialize, Deserialize};
use super::schema::tests;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Test {
    pub(crate) id: i32,
    pub name: String,
    pub color: i32,
    pub document_id: String,
    pub size: i32,
    pub comment: String,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tests"]
pub struct NewTest {
    pub name: String,
    pub color: i32,
    pub document_id: String,
    pub size: i32,
    pub comment: String,
    pub user_id: String,
    pub user_name: String,
}
