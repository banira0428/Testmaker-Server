use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub(crate) id: u64,
    name: String,
    color: u8,
    document_id: String,
    size: u64,
    comment: String,
    user_id: String,
    user_name: String
}