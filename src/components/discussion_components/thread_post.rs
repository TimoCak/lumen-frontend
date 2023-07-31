use stylist::yew::Global;
use yew::prelude::*;
use crate::{components::ui::{input_field::InputFieldComponent, button::ButtonComponent}, style::thread_post_style::get_thread_post_style};

#[function_component]
pub fn ThreadPostComponent() -> Html {

    let title = use_state(|| "".to_owned());
    let title_setter = title.setter();

    let on_title_entry = Callback::from(move |title: String| {
        title_setter.set(title);
    });

    html! {
        <>  
            <Global css={get_thread_post_style()} />
            <div  class={"thread-post-container"}> 
                <div class={"input-field-container"}> 
                    <InputFieldComponent on_entry={on_title_entry} label={"Title"} input_type={"text"}/>
                </div>
                <textarea> </textarea>
                <div class={"button-container"}> 
                    <ButtonComponent size={"20px"} text={"post"}/>
                </div>
                //Dropdown for categories!
            </div>
        </>
    }
}