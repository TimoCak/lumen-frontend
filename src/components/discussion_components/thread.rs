use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use crate::{apis::backend_api::Backend, compare_uid_to_logged_in_uid, components::{
    discussion_components::post_list::PostListComponent, ui::{button::ButtonComponent, input_field::InputFieldComponent},
}, models::thread::Thread, style::set_styles::set_component_style, get_logged_in_user};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::models::post::{Post, PostForm};
use crate::models::user::UserStored;
use crate::router::Route;

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

    //post credentials
    let post_title = use_state(|| String::default());
    let post_title_setter = post_title.setter();
    let post_text = use_state(|| String::default());
    let post_text_setter = post_text.setter();

    let created_post = use_state(|| Post::default());
    let created_post_setter = created_post.setter();

    //define navigator
    let navigator = use_navigator().unwrap();


    let onclick = Callback::from(move |_| {
        Backend::delete_thread(id.clone());
        navigator.push(&Route::Discussions);
    });

    let onchange = Callback::from(move |e: Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();

        post_text_setter.set(value);
    });

    //callback function to save input value
    let ont_post_title_entry = Callback::from(move |title: String| {
        post_title_setter.set(title);
    });

    //get current logged in user
    let current_user: UserStored = get_logged_in_user();

    let post_form: PostForm = PostForm {
        thread_id: id.clone(),
        author: current_user.username,
        title: (*post_title).clone(), //need to set from user
        text: (*post_text).clone(), //need to set  form user
    };

    let oncreatepost = Callback::from(move |_| {
        Backend::create_post(post_form.clone(), created_post_setter.clone());
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
                    <img onclick={onclick} id={"thread-delete-button"} src={"./assets/images/delete_icon.png"} width={"50"} height={"50"}/>
                }
            </div>
            <div id="create-post-form">
                <InputFieldComponent label={"title"} input_type={"text"} on_entry={ont_post_title_entry}/>
                <textarea placeholder={"express your opinion"} id={"editor"} onchange={onchange}></textarea>
                <div onclick={oncreatepost}
                id={"create-post-button"}>
                    <ButtonComponent size={"18px"} text={"create a post"} color={"white"} width={"10%"}/>
                </div>
            </div>
            <PostListComponent thread_id={id.clone()}/>
        </>

    }
}
