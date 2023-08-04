use reqwasm::http::Request;
use serde::Serialize;
use yew::{platform::spawn_local, UseStateSetter};

use crate::get_backend_url;

#[derive(Serialize)]
pub struct ThreadForm {
    pub author: String,
    pub title: String,
    pub text: String,
    pub categories: Vec<String>,
}

pub fn post_thread(thread_form: ThreadForm, status_message: UseStateSetter<String>) {
    spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/threads", backend_url);

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&thread_form).unwrap())
            .send()
            .await
            .unwrap();

        //let fetched_thread = response.text().await.unwrap();

        //use_state_handle_setter.set(fetched_user.clone())

        if response.status() == 201 {
            status_message.set("succesfully posted thread!".to_string());
        } else {
            status_message.set("something went wrong during posting this thread!".to_string());
        }
    });
}
