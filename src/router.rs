use crate::{
    components::{
        discussion_components::thread::ThreadComponent, discussions::DiscussionsComponent,
        home::HomeComponent, login::LoginComponent, register::RegisterComponent,
        showcase::ShowcaseComponent,
    },
    style::set_styles::set_component_style,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/showcase")]
    Showcase,

    #[at("/discussions")]
    Discussions,

    #[at("/login")]
    Login,

    #[at("/register")]
    Register,

    #[at("/discussions/:id")]
    Thread { id: i32 },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            set_component_style(vec![String::from("home_style.css")]);
            html! { <><HomeComponent /></>
            }
        }
        Route::Showcase => html! { <><ShowcaseComponent /></>},
        Route::Discussions => {
            set_component_style(vec![
                String::from("discussions_style.css"),
                String::from("thread_list_style.css"),
                String::from("thread_post_style.css"),
                String::from("button_style.css"),
            ]);
            html! { <><DiscussionsComponent /></>}
        }
        Route::Login => {
            set_component_style(vec![
                String::from("login_style.css"),
                String::from("input_field_style.css"),
                String::from("button_style.css"),
            ]);
            html! { <><LoginComponent /></>}
        }
        Route::Register => {
            set_component_style(vec![
                String::from("register_style.css"),
                String::from("input_field_style.css"),
                String::from("button_style.css"),
            ]);
            html! {<><RegisterComponent /></>}
        }
        Route::Thread { id } => {
            set_component_style(vec![
                String::from("thread_style.css"),
                String::from("button_style.css"),
                String::from("post_list_style.css"),
                String::from("post_style.css"),
            ]);
            html! {<><ThreadComponent id={id}/></>}
        }
        Route::NotFound => html! { <><h3>{"404 NOT FOUND!"}</h3></>},
    }
}
