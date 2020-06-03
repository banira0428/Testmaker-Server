use serde::{Serialize, Deserialize};
use super::schema::tests;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Test {
    pub(crate) id: i32,
    pub name: String,
    color: i32,
    document_id: String,
    size: i32,
    comment: String,
    user_id: String,
    user_name: String
}

#[derive(Insertable)]
#[table_name="tests"]
pub struct NewTest<'a> {
    pub name: &'a str,
    pub color: &'a i32,
    pub document_id: &'a str,
    pub size: &'a i32,
    pub comment: &'a str,
    pub user_id: &'a str,
    pub user_name: &'a str
}
