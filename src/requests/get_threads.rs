use std::time::SystemTime;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::{platform::spawn_local, UseStateSetter};

use crate::get_backend_url;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Thread {
    pub id: i32,
    pub author: String,
    pub created_at: SystemTime,
    pub title: String,
    pub text: String,
    pub likes: i32,
    pub dislikes: i32,
    pub categories: Vec<String>,
}

pub fn get_threads(thread_list: UseStateSetter<Vec<Thread>>) {
    spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/threads", backend_url);

        /* need to handle unwraps */
        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<Vec<Thread>>().await.unwrap();

        thread_list.set(response_text);
    });
}
