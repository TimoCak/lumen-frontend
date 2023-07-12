use crate::components::ui::button::ButtonComponent;
use crate::components::ui::input_field::InputFieldComponent;
use crate::{router::Route, style::register_style::get_register_style};
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component]
pub fn RegisterComponent() -> Html {
    html! {
        <>
            <Global css={get_register_style()} />
            <div class={classes!("login-form")}>
                <InputFieldComponent label={"username:"} input_type={"text"}/>
                <InputFieldComponent label={"email:"} input_type={"email"}/>
                <InputFieldComponent label={"password:"} input_type={"password"}/>
                <InputFieldComponent label={"repeat password:"} input_type={"password"}/>
                <div class={"button-container"}>
                    <ButtonComponent size={"20px"} text={"sign up"}/>
                    <p><Link<Route> to={Route::Login}>{"Already signed up?"}</Link<Route>></p>
                </div>
            </div>
        </>
    }
}
