use chrono::DateTime;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::models::thread::Thread;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub threads: Vec<Thread>,
}

#[function_component]
pub fn ThreadListComponent(props: &Props) -> Html {
    html! {
        <>
        <div id={"thread-list-container"}>
        {
            props.threads.iter().map(|thread| {
                html!{
                    <>
                    <Link<Route> to={Route::Thread{id: thread.clone().id}}>
                    <div class={"thread-info"}>
                        <h3 key={thread.id}>
                            {thread.title.clone()}
                        </h3>
                        <p class={"author"}>{thread.clone().author}</p>
                        <p class={"timestamp"}>{DateTime::from_timestamp(thread.clone().created_at.secs_since_epoch.into(), 0
                        ).unwrap().to_rfc2822()}</p>
                    </div>
                    </Link<Route>>
                    </>
                }
            }).rev().collect::<Html>()
        }
        </div>
        </>
    }
}
