use yew::prelude::*;
use yew_router::prelude::*;
use lumen_frontend::components::header::HeaderComponent;
use lumen_frontend::style::app_style::get_app_style;
use stylist::yew::Global;
use lumen_frontend::router::{switch, Route};
use lumen_frontend::js::log;


#[function_component]
fn App() -> Html {
    /*let location = use_route::<Route>().unwrap_or_default();
    let location_2 = use_location().unwrap();
    log(format!("{:?}", location));*/
    html!{
        <>  
            <BrowserRouter>
                <Global css={get_app_style()}/>
                <HeaderComponent />
                <Switch<Route> render={switch} />
                <footer><a target={"_blank"} href={"https://github.com/TimoCak/lumen-frontend"}>{"Github"}</a></footer>
            </BrowserRouter>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
