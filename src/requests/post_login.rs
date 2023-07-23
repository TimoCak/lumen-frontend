use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::window;
use yew::UseStateSetter;
use yew_router::prelude::Navigator;

use crate::get_frontend_url;
use crate::js::setItem;
use crate::{get_backend_url, router::Route};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

pub fn post_login(
    username: &String,
    password: &String,
    navigator: Navigator,
    response_text: UseStateSetter<String>
) {
    let user = User {
        username: username.to_owned(),
        password: password.to_owned(),
    };

    let location = window().unwrap().location();

    wasm_bindgen_futures::spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/sign-in", backend_url);

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&user).unwrap())
            .send()
            .await
            .unwrap();

        let response_body = response.text().await.unwrap();

        if response.status() == 400 {
            response_text.set("please fill out all fields!".to_owned());
        } else if response.status() == 401 {
            response_text.set("wrong username or password!".to_owned());
        } else if response.status() == 200 {
            setItem("currentUser".to_owned(), response_body.into());
            navigator.push(&Route::Home);
            location.set_href(&get_frontend_url()).unwrap();
        }
        
    });
}
