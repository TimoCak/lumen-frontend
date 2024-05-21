use crate::{
    apis::backend_api::Backend,
    compare_uid_to_logged_in_uid,
    components::{
        discussion_components::post_list::PostListComponent, ui::button::ButtonComponent,
    },
    get_location,
    models::thread::Thread,
    style::set_styles::set_component_style,
    FRONTEND_URL,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component]
pub fn ThreadComponent(props: &Props) -> Html {
    //set components style
    let style_vec: Vec<String> = vec![String::from("thread_style.css"), String::from("button_style.css"), String::from("post_list_style.css")];
    set_component_style(style_vec);

    let id = props.id;

    let thread = use_state(|| Thread::new());
    let thread_setter = thread.setter();

    let location = get_location();

    let onclick = Callback::from(move |_| {
        Backend::delete_thread(id.clone());
        location
            .set_href(&format!("{}/discussions", FRONTEND_URL))
            .unwrap();
    });
    use_effect_with((), move |_| {
        Backend::get_thread_by_id(id.clone(), thread_setter)
    });

    html! {
        <>

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
            <PostListComponent thread_id={id.clone()}/>
        </>

    }
}
