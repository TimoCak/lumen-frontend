use yew::prelude::*;
use yew_router::prelude::*;
use lumen_frontend::components::header::HeaderComponent;
use lumen_frontend::router::{switch, Route};


#[function_component]
fn App() -> Html {

    html!{
        <>  
            <BrowserRouter>
                <link rel={"stylesheet"} href={"/assets/css/app_style.css"} />
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
