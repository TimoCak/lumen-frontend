use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub size: String,
}

#[function_component]
pub fn ButtonComponent(props: &Props) -> Html {
    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/button_style.css"}/>
            <button style={format!("font-size: {}",props.size.clone())}>{props.text.clone()}</button>
        </>
    }
}
