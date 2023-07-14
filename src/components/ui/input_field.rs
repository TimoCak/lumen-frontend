use stylist::yew::Global;
use yew::prelude::*;
use crate::style::input_field_style::get_input_field_style;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub input_type: String,
    pub on_entry: Callback<String>,
}

#[function_component]
pub fn InputFieldComponent(props: &Props) -> Html {

    let entry = props.on_entry.clone();

    let onchange = {
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            entry.emit(value)
        })
    };
    
    html! {
        <>  
            <Global css={get_input_field_style()} />
            <div class={classes!("login-field")}>
                <label>{props.label.clone()}</label>
                <input onchange={onchange} type={props.input_type.clone()}/>
            </div>
        </>
    }
}