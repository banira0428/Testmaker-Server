use serde::{Serialize, Deserialize};

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