use yew::prelude::*;
use lumen_frontend::components::header::HeaderComponent;
use lumen_frontend::style::app_style::get_app_style;
use stylist::yew::Global;

#[function_component]
fn App() -> Html {
    html!{
        <>  
            <Global css={get_app_style()}/>
            <HeaderComponent />
            <h1>{"Hello Lumen!"}</h1>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
