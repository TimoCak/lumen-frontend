use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::UseStateSetter;
use yew_router::prelude::Navigator;

use crate::{get_backend_url, router::Route};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String
}

pub fn post_login(username: &String, password: &String, navigator: Navigator, response_text: UseStateSetter<String>) {

    let user = User {
        username: username.to_owned(),
        password: password.to_owned()
    };

    wasm_bindgen_futures::spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/sign-in", backend_url);

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&user).unwrap())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        response_text.set(response.clone());

        if response.eq("succesfully logged in!") {
            navigator.push(&Route::Home);
        }
    });
}