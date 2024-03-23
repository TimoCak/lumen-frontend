use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};
use crate::{apis::backend_api::Backend, components::discussion_components::post_list::PostListComponent, models::thread::Thread, compare_uid_to_logged_in_uid, components::ui::button::ButtonComponent, get_location, FRONTEND_URL};
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component]
pub fn ThreadComponent(props: &Props) -> Html {
    let id = props.id;

    let thread = use_state(|| Thread::new());
    let thread_setter = thread.setter();

    let location = get_location();

    let onclick = Callback::from(move |_| {
        Backend::delete_thread(id.clone());
        location.set_href(&format!("{}/discussions", FRONTEND_URL)).unwrap();
    });
    use_effect_with(
        (), move |_| Backend::get_thread_by_id(id.clone(), thread_setter),
    );

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/thread_style.css"} />
            <div class={"thread-container"}>
                <h3 class={"thread-title"}>{thread.title.clone()}</h3>
                <div>{"by "}<a href={""}>{thread.author.clone()}</a></div>
                <div class={"thread-text"}>{thread.text.clone()}</div>
                if compare_uid_to_logged_in_uid(&thread.author.clone()) {
                    <div onclick={onclick}>
                        <ButtonComponent size={"12px"} text={"delete"} color={"red"} width={"10%"}/>
                    </div>
                }
            </div>
            <div class={"thread-list-container"}> 
                <PostListComponent thread_id={id.clone()}/>
            </div>
        </>

    }
}
