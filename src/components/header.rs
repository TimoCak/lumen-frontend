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
                <Link<Route> to={Route::Home}><h1>{"Lumen"}</h1></Link<Route>>
                <Link<Route> to={Route::Showcase}><h2>{"Showcase"}</h2></Link<Route>>
                <Link<Route> to={Route::Discussions}><h2>{"Discussions"}</h2></Link<Route>>
            </div>
        </>
    }
}