use yew::prelude::*;

use crate::requests::get_threads::Thread;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub thread: Thread,
}

#[function_component]
pub fn ThreadComponent(props: &Props) -> Html {
    
    let thread = props.thread.clone(); 

    html! {
        <>
            <div> 
                <ul> 
                    <li>{thread.id}</li>
                    <li>{thread.author}</li> 
                    <li>{thread.title}</li> 
                    <li>{thread.text}</li> 
                    <li>{thread.likes}</li>
                    <li>{thread.dislikes}</li>
                    <li>{thread.categories.len()}</li>   
                </ul>
            </div>

        </>

    }
}
