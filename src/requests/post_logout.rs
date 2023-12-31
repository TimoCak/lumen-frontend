use reqwasm::http::Request;
use web_sys::window;
use yew::platform::spawn_local;
use yew::UseForceUpdateHandle;
use yew_router::prelude::Navigator;

use crate::get_backend_url;
use crate::router::Route;

pub fn post_logout(navigator: Navigator, update: UseForceUpdateHandle) {
    let storage = window().unwrap().session_storage().unwrap().unwrap();

    spawn_local(async move {
        let backend_url = get_backend_url();
        let url = format!("{}/sign-out", backend_url);

        let response = Request::post(&url).send().await.unwrap();

        if response.status() == 200 {
            storage.clear().unwrap();
            navigator.push(&Route::Home);
            update.force_update();
        }
    });
}
