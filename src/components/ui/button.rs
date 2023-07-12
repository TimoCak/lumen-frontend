use yew::prelude::*;
use crate::style::button_style::get_button_style;
use stylist::yew::Global;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub size: String,
}

#[function_component]
pub fn ButtonComponent(props: &Props) -> Html {
    html! {
        <>  
            <Global css={get_button_style()} />
            <button style={format!("font-size: {}",props.size.clone())}>{props.text.clone()}</button>
        </>
    }
}