use yew::prelude::*;

use crate::requests::get_threads::get_threads;
use crate::components::discussion_components::thread::ThreadComponent;

#[function_component]
pub fn ThreadListComponent() -> Html {
    let threads = use_state(|| vec![]);
    let threads_setter = threads.setter();

    use_effect_with_deps(move |()| get_threads(threads_setter), ());

    html! {
        <>
        <div id={"introductions"}>
        {
            threads.iter().map(|thread| {
                html!{<div key={thread.id}><ThreadComponent thread={thread.clone()}/></div>}
            }).collect::<Html>()
        }
        </div>

        </>

    }
}