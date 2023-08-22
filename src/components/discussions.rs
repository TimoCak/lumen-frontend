use crate::components::{
    discussion_components::{thread_list::ThreadListComponent, thread_post::ThreadPostComponent},
    ui::button::ButtonComponent,
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

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/discussions_style.css"}/>
            <div class={"discussions-container"}>
                <div >
                    <div class={"toggle-button-container"} onclick={ontoggle}>
                        <ButtonComponent text={"post thread"} size={"16"}/>
                    </div>
                    if toggle_display {
                        <ThreadPostComponent />
                    }
                </div>
                <ThreadListComponent />
            </div>
        </>
    }
}
