use serde::{Deserialize, Serialize};

use super::Timestamp;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Thread {
    pub id: i32,
    pub author: String,
    pub created_at: Timestamp,
    pub title: String,
    pub text: String,
    pub likes: i32,
    pub dislikes: i32,
    pub categories: Vec<String>,
}

#[derive(Serialize)]
pub struct ThreadForm {
    pub author: String,
    pub title: String,
    pub text: String,
    pub categories: Vec<String>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            id: -1,
            author: "".to_owned(),
            created_at: Timestamp::default(),
            title: "".to_owned(),
            text: "".to_owned(),
            likes: 0,
            dislikes: 0,
            categories: vec![],
        }
    }
}
