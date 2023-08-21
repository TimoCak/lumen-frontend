use yew::{prelude::*, platform::spawn_local};

use crate::{
    components::discussion_components::post_list::PostListComponent,
    requests::get_threads::{get_thread_by_id, Thread},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component]
pub fn ThreadComponent(props: &Props) -> Html {
    let id = props.id;

    let thread = use_state(|| Thread::new());
    let thread_setter = thread.setter();

    use_effect_with_deps(
        move |()| spawn_local(async move { get_thread_by_id(thread_setter, id.clone()).await }),
        (),
    );

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/thread_style.css"} />
            <div class={"thread-container"}>
                <h3 class={"thread-title"}>{thread.title.clone()}</h3>
                <div>{"by "}<a href={""}>{thread.author.clone()}</a></div>
                <div class={"thread-text"}>{thread.text.clone()}</div>
            </div>
            <PostListComponent thread_id={id.clone()}/>
        </>

    }
}
