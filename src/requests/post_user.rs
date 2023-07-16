use serde::{Serialize, Deserialize};
use serde_json;
use reqwasm::http::Request;
use crate::get_backend_url;
use crate::js::log;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserResponse {
    id: i32,
    username: String,
    email: String,
    password: String,
    role: String,
}

/* woanders auslagern */


/* extrat Ordner mit HTTP-Requests erstellen */
pub fn post_user(username: String, email: String, password: String) {
    
    let user = User {
        username,
        email,
        password,
    };

    wasm_bindgen_futures::spawn_local(async move {
        
        let backend_url = get_backend_url();
        let url = format!("{}/sign-up", backend_url);

        let fetched_user = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&user).unwrap())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        log(format!("{}", fetched_user));
    });
    
    
    /*serde_json::from_str(&response).expect("couldn't deserialize into UserResponse")  */
}