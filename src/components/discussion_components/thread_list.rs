use yew::prelude::*;
use yew_router::prelude::Link;

use crate::requests::get_threads::get_threads;
use crate::router::Route;

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
                html!{
                    <h3 key={thread.id}>
                    <Link<Route> to={Route::Thread{id: thread.id}}><li>{thread.title.clone()}</li></Link<Route>>
                    </h3>}
            }).collect::<Html>()
        }
        </div>

        </>

    }
}
