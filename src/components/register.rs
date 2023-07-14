use crate::components::ui::button::ButtonComponent;
use crate::components::ui::input_field::InputFieldComponent;
use crate::js::log;
use crate::{router::Route, style::register_style::get_register_style};
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component]
pub fn RegisterComponent() -> Html {

    let username_entry = use_state(|| "".to_owned());
    let username_entry_setter = username_entry.setter();

    let email_entry = use_state(String::default);


    let password_entry = use_state(String::default);
    let repeated_password_entry = use_state(String::default);

    let username_value_visual = (*username_entry).clone();

    let on_username_entry = Callback::from(move |username: String| {
        username_entry_setter.set(username);
    });

    let on_email_entry = Callback::from(move |email: String| {
        let greeting = format!("Hey, {}", email);
    });

    let on_password_entry = Callback::from(move |password: String| {
        let greeting = format!("Hey, {}", password);
    });

    let on_repeated_password_entry = Callback::from(move |repeated_password: String| {
        let greeting = format!("Hey, {}", repeated_password);
    });

    let onclick = {
        move |_| {
            log("CLICK!".to_string());
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
