use yew::prelude::*;

use crate::requests::get_threads::{get_thread_by_id, Thread};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component]
pub fn ThreadComponent(props: &Props) -> Html {
    let id = props.id;

    let thread = use_state(|| Thread::new());
    let thread_setter = thread.setter();
    use_effect_with_deps(move |()| get_thread_by_id(thread_setter, id.clone()), ());

    html! {
        <>
            <div>
                <h3>{thread.title.clone()}</h3>
                <div>{thread.text.clone()}</div>
            </div>
        </>

    }
}
