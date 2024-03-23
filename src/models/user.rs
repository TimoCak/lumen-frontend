use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserStored {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUser {
    pub username: String,
    pub password: String,
}