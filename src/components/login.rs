use crate::components::ui::button::ButtonComponent;
use crate::components::ui::input_field::InputFieldComponent;
use crate::requests::post_login::post_login;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

#[function_component]
pub fn LoginComponent() -> Html {
    let navigator = use_navigator().unwrap();

    let username_entry = use_state(|| "".to_owned());
    let username_entry_setter = username_entry.setter();

    let password_entry = use_state(|| "".to_owned());
    let password_entry_setter = password_entry.setter();

    let message = use_state(|| "".to_owned());
    let message_visible = (*message).clone();

    //let user

    let on_username_entry = Callback::from(move |username: String| {
        username_entry_setter.set(username);
    });

    let on_password_entry = Callback::from(move |password: String| {
        password_entry_setter.set(password);
    });

    let onclick = Callback::from(move |_| {
        post_login(
            &username_entry,
            &password_entry,
            navigator.clone(),
            message.setter(),
        );
    });

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/login_style.css"} />
            <div class={classes!("login-form")}>
                <InputFieldComponent on_entry={on_username_entry} label={"username:"} input_type={"text"}/>
                <InputFieldComponent on_entry={on_password_entry} label={"password:"} input_type={"password"}/>
                <div class={"button-container"}>
                    <div onclick={onclick}>
                        <ButtonComponent size={"20px"} text={"sign in"}/>
                    </div>
                    <p><Link<Route> to={Route::Register}>{"Not signed up yet?"}</Link<Route>></p>
                </div>
                <p class={"error-message"}>{message_visible}</p>
            </div>
        </>
    }
}
