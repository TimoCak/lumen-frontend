use stylist::yew::Global;
use yew::prelude::*;
use crate::style::input_field_style::get_input_field_style;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub input_type: String,
}

#[function_component]
pub fn InputFieldComponent(props: &Props) -> Html {
    html! {
        <>  
            <Global css={get_input_field_style()} />
            <div class={classes!("login-field")}>
                <label>{props.label.clone()}</label>
                <input type={props.input_type.clone()}/>
            </div>
        </>
    }
}