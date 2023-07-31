use stylist::yew::Global;
use yew::prelude::*;
use crate::{components::discussion_components::{thread_list::ThreadListComponent, thread_post::ThreadPostComponent}, style::discussions_style::get_discussions_style};

#[function_component]
pub fn DiscussionsComponent() -> Html {

    let toggle_form = use_state(|| false);

    let toggle_form_setter = toggle_form.setter();

    let toggle_display = (*toggle_form).clone();

    let ontoggle = Callback::from(move |_| {

        let toggle = !toggle_display;

        toggle_form_setter.set(toggle);
    });

    html! {
        <>  
            <Global css={get_discussions_style()} />
            <div class={"discussions-container"}>
                <div >
                    <button onclick={ontoggle}>{"KLICK"}</button>
                    if toggle_display {
                        <ThreadPostComponent />
                    } 
                </div>
                <ThreadListComponent />
            </div>
        </>
    }
}