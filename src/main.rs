use lumen_frontend::components::header::HeaderComponent;
use lumen_frontend::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <> 
            <BrowserRouter>
                <HeaderComponent />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
