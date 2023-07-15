use crate::components::ui::button::ButtonComponent;
use crate::components::ui::input_field::InputFieldComponent;
use crate::get_backend_url;
use crate::js::log;
use crate::{router::Route, style::register_style::get_register_style};
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::Link;
use serde::{Serialize, Deserialize};
use serde_json;
use reqwasm::http::Request;

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
fn post_user(username: String, email: String, password: String) {
    
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

#[function_component]
pub fn RegisterComponent() -> Html {

    let username_entry = use_state(|| "".to_owned());
    let username_entry_setter = username_entry.setter();

    let email_entry = use_state(|| "".to_owned());
    let email_entry_setter = email_entry.setter();


    let password_entry = use_state(String::default);
    let password_entry_setter = password_entry.setter(); 

    let repeated_password_entry = use_state(String::default);
    let repeated_password_entry_setter = repeated_password_entry.setter();

    let username_value_visual = (*username_entry).clone();

    let on_username_entry = Callback::from(move |username: String| {
        username_entry_setter.set(username);
    });

    let on_email_entry = Callback::from(move |email: String| {
        email_entry_setter.set(email);
    });

    let on_password_entry = Callback::from(move |password: String| {
        password_entry_setter.set(password);
    });

    let on_repeated_password_entry = Callback::from(move |repeated_password: String| {
        repeated_password_entry_setter.set(repeated_password);
    });

    let onclick = {
        move |_| {
            post_user((*username_entry).clone(), (*email_entry).clone(), (*password_entry).clone());
        }
    };

    html! {
        <>
            <Global css={get_register_style()} />
            <div class={classes!("login-form")}>
                <InputFieldComponent on_entry={on_username_entry} label={"username:"} input_type={"text"}/>
                <InputFieldComponent on_entry={on_email_entry} label={"email:"} input_type={"email"}/>
                <InputFieldComponent on_entry={on_password_entry} label={"password:"} input_type={"password"}/>
                <InputFieldComponent on_entry={on_repeated_password_entry} label={"repeat password:"} input_type={"password"}/>
                <div class={"button-container"}>
                     <div onclick={onclick}>
                        <ButtonComponent  size={"20px"} text={"sign up"}/>
                     </div>
                    <p><Link<Route> to={Route::Login}>{"Already signed up?"}</Link<Route>></p>
                </div>
            </div>
        </>
    }
}
