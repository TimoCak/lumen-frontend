use serde::{Serialize, Deserialize};
use serde_json;
use reqwasm::http::Request;
use crate::get_backend_url;
use crate::js::alert;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

pub fn post_user(username: String, email: String, password: String, repeated_password: String) -> String {
    
    let mut output = "";

    let user = User {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
    };

    if repeated_password.eq("") || password.eq("") || email.eq("") || username.eq("") {
        output = "fill out all input fields!";
    } else if !password.eq(repeated_password.as_str()) {
        output = "Passwords do not match!";
    } else {

        output = "post user successed!";

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
                            
            
            if fetched_user.eq("username is already taken!") {
                alert(fetched_user);
            } 
        });
        
    }

    output.to_owned()
    /*serde_json::from_str(&response).expect("couldn't deserialize into UserResponse")  */
}