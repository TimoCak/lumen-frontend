use yew::prelude::*;
use crate::components::discussion_components::thread_list::ThreadListComponent;

#[function_component]
pub fn DiscussionsComponent() -> Html {
    html! {
        <>
            <h3>{"Discussions"}</h3>
            <ThreadListComponent />
        </>
    }
}