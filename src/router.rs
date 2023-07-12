use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::{home::HomeComponent, showcase::ShowcaseComponent, discussions::DiscussionsComponent, 
    login::LoginComponent, register::RegisterComponent};

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

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <><HomeComponent /></>},
        Route::Showcase => html!{ <><ShowcaseComponent /></>},
        Route::Discussions => html!{ <><DiscussionsComponent /></>},
        Route::Login => html!{ <><LoginComponent /></>},
        Route::Register => html!{<><RegisterComponent /></>},
        Route::NotFound => html!{ <><h3>{"404 NOT FOUND!"}</h3></>},
    }
}