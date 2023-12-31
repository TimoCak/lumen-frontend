use crate::components::{
    discussion_components::thread::ThreadComponent, discussions::DiscussionsComponent,
    home::HomeComponent, login::LoginComponent, register::RegisterComponent,
    showcase::ShowcaseComponent,
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
        Route::Home => html! { <><HomeComponent /></>},
        Route::Showcase => html! { <><ShowcaseComponent /></>},
        Route::Discussions => html! { <><DiscussionsComponent /></>},
        Route::Login => html! { <><LoginComponent /></>},
        Route::Register => html! {<><RegisterComponent /></>},
        Route::Thread { id } => html! {<><ThreadComponent id={id}/></>},
        Route::NotFound => html! { <><h3>{"404 NOT FOUND!"}</h3></>},
    }
}
