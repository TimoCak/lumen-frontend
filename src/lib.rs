use web_sys::{window, Storage};

pub mod components;
pub mod apis;
pub mod router;
pub mod models;

pub const BACKEND_URL: &str = "http://127.0.0.1:8081/api";
pub const FRONTEND_URL: &str = "http://127.0.0.1:8080";

pub fn is_user_set() -> bool {
    let store = window().unwrap().session_storage().unwrap().unwrap();

    let option_string = store.get_item("currentUser").unwrap();

    let stored_user = match option_string {
        Some(v) => v,
        None => "".to_owned(),
    };
    println!("{:?}", &stored_user);
    if !stored_user.eq("") {
        return true;
    }

    return false;
}

pub fn get_session_storage() -> Storage {
    window().unwrap().session_storage().unwrap().unwrap()
}
