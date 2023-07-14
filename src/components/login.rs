use yew::prelude::*;
use yew_router::prelude::Link;
use crate::{style::login_style::get_login_style, router::Route};
use stylist::yew::Global;
use crate::components::ui::input_field::InputFieldComponent;
use crate::components::ui::button::ButtonComponent;

#[function_component]
pub fn LoginComponent() -> Html {

    let on_username_entry = Callback::from(move |username: String|{
        let greeting = format!("Hey, {}", username);
        /* save here to variable */
    });

    let on_password_entry = Callback::from(move |password: String|{
        let greeting = format!("Hey, {}", password);
        /* save here to variable! */
    });

    html! {
        <>  
            <Global css={get_login_style()} />
            <div class={classes!("login-form")}>
                <InputFieldComponent on_entry={on_username_entry} label={"username:"} input_type={"text"}/>
                <InputFieldComponent on_entry={on_password_entry} label={"password:"} input_type={"password"}/>
                <div class={"button-container"}>
                    <ButtonComponent size={"20px"} text={"sign in"}/>
                    <p><Link<Route> to={Route::Register}>{"Not signed up yet?"}</Link<Route>></p>
                </div>
            </div>
            
        </>
    }
}
