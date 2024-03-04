use crate::{
    apis::backend_api::Backend, components::ui::{button::ButtonComponent, input_field::InputFieldComponent}, models::{thread::ThreadForm, user::UserStored}
};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component]
pub fn ThreadPostComponent() -> Html {
    let title = use_state(|| "".to_owned());
    let title_setter = title.setter();

    let text = use_state(|| "".to_owned());
    let text_setter = text.setter();

    let onchange = Callback::from(move |e: Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();

        text_setter.set(value);
    });

    let status_message = use_state(|| "".to_owned());
    let display_status_message = (*status_message).clone();

    let on_title_entry = Callback::from(move |title: String| {
        title_setter.set(title);
    });

    let onclick = Callback::from(move |_| {
        let store = window().unwrap().session_storage().unwrap().unwrap();

        let user_stored: UserStored = serde_json::from_str(
            &store
                .get_item("currentUser")
                .unwrap()
                .expect("failed to get curerntUser from storage!"),
        )
        .unwrap();
        let thread_form = ThreadForm {
            author: user_stored.username.clone(),
            title: (*title).clone(),
            text: (*text).clone(),
            categories: vec![], //need further implementation! -> select option values
        };
        Backend::create_thread(thread_form, status_message.setter());
    });

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/thread_post_style.css"} />
            <div  class={"thread-post-container"}>
                <div class={"input-field-container"}>
                    <InputFieldComponent placeholder={"give your thread a title"} on_entry={on_title_entry} label={"Title"} input_type={"text"} size={30}/>
                </div>

                <span>{"Text"}</span>
                <textarea placeholder={"express your opinion"} id={"editor"} onchange={onchange}></textarea>
        
                <div onclick={onclick} class={"button-container"}>
                    <ButtonComponent size={"20px"} text={"add discussion"}/>
                </div>
                //Dropdown for categories!
                <p class={"status-message"}>{display_status_message}</p>
            </div>
        </>
    }
}
