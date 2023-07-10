use yew::prelude::*;
use yew_router::prelude::*;
use lumen_frontend::components::header::HeaderComponent;
use lumen_frontend::style::app_style::get_app_style;
use stylist::yew::Global;
use lumen_frontend::router::{switch, Route};


#[function_component]
fn App() -> Html {
    html!{
        <>  
            <Global css={get_app_style()}/>
            <HeaderComponent />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
