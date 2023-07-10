use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::home::HomeComponent;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <><HomeComponent /></>},
        Route::NotFound => html!{ <><h3>{"404 NOT FOUND!"}</h3></>}
    }
}