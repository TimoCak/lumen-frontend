use reqwasm::http::Request;
use serde::Serialize;
use yew::{platform::spawn_local, UseStateSetter};
use base64::{Engine as _, engine::general_purpose};
use web_sys::window;
use log::info;

use crate::get_backend_url;
use crate::UserStored;

#[derive(Serialize)]
pub struct ThreadForm {
    pub author: String,
    pub title: String,
    pub text: String,
    pub categories: Vec<String>,
}

pub fn post_thread(thread_form: ThreadForm, status_message: UseStateSetter<String>) {

    let storage = window().unwrap().session_storage().unwrap().unwrap();
    
    spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/threads", backend_url);

        let user_stored: UserStored = serde_json::from_str(
            &storage
                .get_item("currentUser")
                .unwrap()
                .expect("failed to get curerntUser from storage!"),
        )
        .unwrap();

        let auth = general_purpose::STANDARD.encode(format!("{}:{}", &user_stored.username, &user_stored.password));
        info!("{:?}", auth.clone());
        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {}",&auth).as_ref())
            .header("withCredentials", "true")
            .body(serde_json::to_string(&thread_form).unwrap())
            .send()
            .await
            .unwrap();

        if response.status() == 201 {
            status_message.set("succesfully posted thread!".to_string());
        } else {
            status_message.set("something went wrong during posting this thread!".to_string());
        }
    });
}
