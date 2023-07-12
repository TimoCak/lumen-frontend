use yew::prelude::*;
use stylist::yew::Global;
use yew_router::prelude::Link;
use crate::{style::header_style::get_header_style, router::Route};

#[function_component]
pub fn HeaderComponent() -> Html {

    html! {
        <>  
            <Global css={get_header_style()} />
            <div class={classes!("header")}>
                <Link<Route> to={Route::Home} classes={classes!("header-link")}><h1>{"Lumen"}</h1></Link<Route>>
                <Link<Route> to={Route::Showcase} classes={classes!("header-link")}><h2>{"Showcase"}</h2></Link<Route>>
                <Link<Route> to={Route::Discussions} classes={classes!("header-link")}><h2>{"Discussions"}</h2></Link<Route>>
                <Link<Route> to={Route::Login} classes={classes!("header-link")}><img class={classes!("filter-light")} src={"./assets/sign_in.svg"}/></Link<Route>>
            </div>
        </>
    }
}