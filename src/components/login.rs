use yew::prelude::*;
use yew_router::prelude::Link;
use crate::{style::login_style::get_login_style, router::Route};
use stylist::yew::Global;
use crate::components::ui::input_field::InputFieldComponent;
use crate::components::ui::button::ButtonComponent;

#[function_component]
pub fn LoginComponent() -> Html {
    html! {
        <>  
            <Global css={get_login_style()} />
            <div class={classes!("login-form")}>
                <InputFieldComponent label={"username:"} input_type={"text"}/>
                <InputFieldComponent label={"password:"} input_type={"password"}/>
                <div class={"button-container"}>
                    <ButtonComponent size={"20px"} text={"sign up"}/>
                    <p><Link<Route> to={Route::Home}>{"Not signed up yet?"}</Link<Route>></p>
                </div>
            </div>
            
        </>
    }
}
