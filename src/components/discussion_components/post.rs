use yew::{function_component, html, Html, Properties};

use crate::models::post::Post;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: Post,
}

#[function_component]
pub fn PostComponent(props: &Props) -> Html {
    html! {
        <>
        <div class={"post-container"}>
            <div class={"author"}><a href={""}>{props.post.author.clone()}</a></div>
                <div>
                    {props.post.text.clone()}
                </div>
        </div>    
        </>
    }
}
