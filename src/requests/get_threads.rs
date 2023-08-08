use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::{platform::spawn_local, UseStateSetter};

use crate::get_backend_url;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Timestamp {
    pub secs_since_epoch: u32,
    pub nanos_since_epoch: u32,
}

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

pub async fn get_thread_by_id(thread_list: UseStateSetter<Thread>, id: i32) {
    let backend_url = get_backend_url();
    let url = format!("{}/threads/{}", backend_url, id);

    /* need to handle unwraps */
    let response = Request::get(&url).send().await.unwrap();

    let response_text = response.json::<Vec<Thread>>().await.unwrap();

    thread_list.set(response_text[0].clone());
}

pub fn get_posts_by_thread_id(post_list: UseStateSetter<Vec<Post>>, id: i32) {
    spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/posts/threads/{}", backend_url, id);

        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<Vec<Post>>().await.unwrap();

        post_list.set(response_text);
    });
}
