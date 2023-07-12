use yew::prelude::*;
use yew_router::prelude::Link;
use crate::{style::login_style::get_login_style, router::Route};
use stylist::yew::Global;

#[function_component]
pub fn LoginComponent() -> Html {
    html! {
        <>  
            <Global css={get_login_style()} />
            <div class={classes!("login-form")}>
                <div class={"login-field"}>
                    <label>{"username:"}</label>
                    <input type={"text"}/>
                </div>
                <div class={"login-field"}>
                    <label>{"password:"}</label>
                    <input type={"password"}/>
                </div>
                <div class={"button-container"}>
                    <button>{"sign in"}</button>
                    <p><Link<Route> to={Route::Home}>{"Not signed up yet?"}</Link<Route>></p>
                </div>
            </div>
            
        </>
    }
}
