use yew::prelude::*;
use yew_router::prelude::Link;
use chrono::DateTime;

use crate::requests::get_threads::get_threads;
use crate::router::Route;

#[function_component]
pub fn ThreadListComponent() -> Html {
    let threads = use_state(|| vec![]);
    let threads_setter = threads.setter();

    use_effect_with((), move |_| get_threads(threads_setter));

    html! {
        <>
        <link rel={"stylesheet"} href={"/assets/css/thread_list_style.css"}/>
        <div id={"thread-list-container"}>
        {   
            threads.iter().map(|thread| {
                html!{
                    <>
                    <Link<Route> to={Route::Thread{id: thread.clone().id}}>
                    <div class={"thread-info"}>
                        <h3 key={thread.id}>
                            {thread.title.clone()}
                        </h3>
                        <p class={"author"}>{thread.clone().author}</p>
                        <p class={"timestamp"}>{DateTime::from_timestamp(thread.clone().created_at.secs_since_epoch.into(), 0).unwrap().to_rfc2822()}</p>
                    </div>
                    </Link<Route>>
                    </>                
                } 
            }).collect::<Html>()
        }
        </div>
        </>
    }
}
