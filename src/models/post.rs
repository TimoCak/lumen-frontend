use serde::{Deserialize, Serialize};

use super::Timestamp;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Post {
    pub id: i32,
    pub thread_id: i32,
    pub answer_id: Option<i32>,
    pub author: String,
    pub created_at: Timestamp,
    pub title: String,
    pub text: String,
    pub likes: Option<i32>,
    pub dislikes: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PostForm {
    pub thread_id: i32,
    pub author: String,
    pub title: String,
    pub text: String,
}