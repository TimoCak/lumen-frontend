use yew::prelude::*;
use stylist::yew::Global;
use crate::style::header_style::get_header_style;

#[function_component]
pub fn HeaderComponent() -> Html {

    html! {
        <>  
            <Global css={get_header_style()} />
            <div class={classes!("header")}>
                <h1>{"Lumen"}</h1>
                <h2>{"Showcase"}</h2>
                <h2>{"Discussions"}</h2>
            </div>
        </>
    }
}