use reqwasm::http::Request;
use web_sys::window;
use yew_router::prelude::Navigator;

use crate::{get_backend_url, get_frontend_url};
use crate::router::Route;

pub fn post_logout(navigator: Navigator) {
    let storage = window().unwrap().session_storage().unwrap().unwrap();

    let location = window().unwrap().location();

    wasm_bindgen_futures::spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/sign-out", backend_url);

        let response = Request::post(&url)
            .send()
            .await
            .unwrap();

        if response.status() == 200 {
            storage.clear().unwrap();
            location.set_href(&get_frontend_url()).unwrap();
            navigator.push(&Route::Home); 
        }
    });
}