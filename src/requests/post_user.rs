use crate::{get_backend_url, router::Route, User};
use reqwasm::http::Request;
use serde_json;
use yew::{platform::spawn_local, UseStateSetter};
use yew_router::prelude::*;

pub fn post_user(
    username: String,
    email: String,
    password: String,
    repeated_password: String,
    use_state_handle_setter: UseStateSetter<String>,
    navigator: Navigator,
) {
    let user = User {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
    };

    if repeated_password.eq("") || password.eq("") || email.eq("") || username.eq("") {
        use_state_handle_setter.set("fill out all input fields!".to_owned());
    } else if !password.eq(repeated_password.as_str()) {
        use_state_handle_setter.set("Passwords do not match!".to_owned());
    } else {
        spawn_local(async move {
            let backend_url = get_backend_url();
            let url = format!("{}/sign-up", backend_url);

            let response = Request::post(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&user).unwrap())
                .send()
                .await
                .unwrap();

            let fetched_user = response.text().await.unwrap();

            use_state_handle_setter.set(fetched_user.clone());

            if response.status() == 200 {
                navigator.push(&Route::Login);
            }
        });
    }
}
