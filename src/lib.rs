use serde::{Deserialize, Serialize};
use web_sys::window;

pub mod components;
pub mod requests;
pub mod router;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStored {
    id: i32,
    username: String,
    email: String,
    password: String,
}

pub fn get_backend_url() -> String {
    String::from("http://127.0.0.1:8081/api")
}

pub fn get_frontend_url() -> String {
    String::from("http://127.0.0.1:8080")
}

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
