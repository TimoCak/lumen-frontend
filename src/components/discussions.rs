use crate::{
    apis::backend_api::Backend,
    components::{
        discussion_components::{
            thread_list::ThreadListComponent, thread_post::ThreadPostComponent,
        },
        ui::button::ButtonComponent,
    },
    models::thread::Thread,
};
use yew::prelude::*;

#[function_component]
pub fn DiscussionsComponent() -> Html {
    let toggle_form = use_state(|| false);

    let toggle_form_setter = toggle_form.setter();

    let toggle_display = (*toggle_form).clone();

    let ontoggle = Callback::from(move |_| {
        let toggle = !toggle_display;

        toggle_form_setter.set(toggle);
    });

    let threads = use_state(|| vec![]);
    let threads_setter = threads.setter();

    let threads_clone = (*threads).clone();

    use_effect_with((), move |_| Backend::get_threads(threads_setter));

    let on_create_thread = Callback::from(move |thread: Thread| {
        if thread.ne(&Thread::default()) {
            let mut updated_threads_list = (*threads).clone();
            updated_threads_list.push(thread);
            threads.setter().set(updated_threads_list);
        }
    });

    html! {
        <>
            <div class={"discussions-container"}>
                <ThreadListComponent threads={threads_clone}/>
                <div class={"action-section"}>
                    <div class={"toggle-button-container"} onclick={ontoggle}>
                        <ButtonComponent text={"➕ New Discussion"} size={"20px"}/>
                    </div>
                    if toggle_display {
                        <ThreadPostComponent {on_create_thread}/>
                    }
                </div>
            </div>
        </>
    }
}
