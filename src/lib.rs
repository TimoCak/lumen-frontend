use base64::{engine::general_purpose, Engine as _};
use web_sys::{window, Document, Location, Storage};
use crate::models::user::UserStored;

pub mod components;
pub mod apis;
pub mod router;
pub mod models;
pub mod style;

pub const BACKEND_URL: &str = "http://127.0.0.1:8081/api";
pub const FRONTEND_URL: &str = "http://127.0.0.1:8080";

pub(crate) fn is_user_set() -> bool {
    let store = window().unwrap().session_storage().unwrap().unwrap();

    let option_string = store.get_item("currentUser").unwrap();

    let stored_user = option_string.unwrap_or_else(|| "".to_owned());
    println!("{:?}", &stored_user);
    if !stored_user.eq("") {
        return true;
    }

    return false;
}

pub(crate) fn get_logged_in_user() -> UserStored {
    let logged_in_user = get_session_storage().get_item("currentUser").unwrap_or_else(|_e| None);

    if logged_in_user.is_none() {
        return UserStored::default();
    }

    serde_json::from_str(&logged_in_user.unwrap()).unwrap_or_else(|_err| UserStored::default())
}

pub(crate) fn get_session_storage() -> Storage {
    window().unwrap().session_storage().unwrap().unwrap()
}

pub(crate) fn get_location() -> Location {
    window().unwrap().location()
    
}

pub(crate) fn get_document() -> Document {
    window().unwrap().document().unwrap()
}

pub(crate) fn compare_uid_to_logged_in_uid(uid: &str) -> bool {
    let current_user: UserStored = get_logged_in_user();

    if current_user.username.ne(uid) {
        return false;
    }

    true
}

pub(crate) fn get_base64_auth_credentials() -> String {
    let current_user = get_logged_in_user();
    let auth = general_purpose::STANDARD.encode(format!(
        "{}:{}",
        &current_user.username, &current_user.password
    ));
    auth
}
