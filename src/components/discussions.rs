use yew::prelude::*;
use crate::components::discussion_components::thread::ThreadComponent;

#[function_component]
pub fn DiscussionsComponent() -> Html {
    html! {
        <>
            <h3>{"Discussions"}</h3>
            <ThreadComponent />
        </>
    }
}