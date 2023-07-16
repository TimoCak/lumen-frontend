use crate::components::ui::button::ButtonComponent;
use crate::components::ui::input_field::InputFieldComponent;
use crate::js::log;
use crate::{router::Route, style::register_style::get_register_style};
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::{Link, use_navigator};
use crate::requests::post_user::post_user;

#[function_component]
pub fn RegisterComponent() -> Html {

    let navigator = use_navigator().unwrap();

    let username_entry = use_state(|| "".to_owned());
    let username_entry_setter = username_entry.setter();

    let email_entry = use_state(|| "".to_owned());
    let email_entry_setter = email_entry.setter();


    let password_entry = use_state(String::default);
    let password_entry_setter = password_entry.setter(); 

    let repeated_password_entry = use_state(String::default);
    let repeated_password_entry_setter = repeated_password_entry.setter();

    let username_value_visual = (*username_entry).clone();

    let error_message = use_state(|| "".to_owned());
    let error_message_setter = error_message.setter();

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
            let message = post_user((*username_entry).clone(), (*email_entry).clone(), (*password_entry).clone(), (*repeated_password_entry).clone());
            error_message_setter.set(message.clone());
            log(message.clone());
            if message.eq("post user successed!") {
                navigator.push(&Route::Login);
            }
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
                <h3>{(*error_message).clone()}</h3>
            </div>
        </>
    }
}