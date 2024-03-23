use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub size: String,
    #[prop_or("white".to_string())]
    pub color: String,
    #[prop_or("100%".to_string())]
    pub width: String,
}

#[function_component]
pub fn ButtonComponent(props: &Props) -> Html {
    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/button_style.css"}/>
            <button class={"button-component"} style={format!("font-size: {}; color: {}; width: {}",props.size.clone(), props.color.clone(), props.width.clone())}>
                {props.text.clone()}</button>
        </>
    }
}
