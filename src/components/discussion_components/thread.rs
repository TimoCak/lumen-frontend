use yew::prelude::*;

use crate::requests::get_threads::get_threads;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
pub fn ThreadComponent() -> Html {
    let threads = use_state(|| vec![]);
    let threads_setter = threads.setter();

    use_effect_with_deps(move |()| get_threads(threads_setter), ());

    html! {
        <>
        <div id={"introductions"}>
        {
            threads.iter().map(|thread| {
                html!{<div key={thread.id}>{ format!("Title: {}!",thread.title) }</div>}
            }).collect::<Html>()
        }
        </div>

        </>

    }
}
