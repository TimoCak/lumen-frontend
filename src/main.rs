use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html!{
        <>
            <h1>{"Hello Lumen!"}</h1>
            <h5>{"Indiegame Platform to showcase own creations and discuss with other devs!"}</h5>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
