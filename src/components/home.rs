use yew::prelude::*;

#[function_component]
pub fn HomeComponent() -> Html {
    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/home_style.css"} />
            <div class={classes!("lumen-intro")}>
                <h3>{"Welcome to Lumen!"}</h3>
            </div>
            <div class={classes!("home-sections")}>
                <div class={classes!("home-section")}>
                    <h3>{"Community"}</h3>
                    <p>{"Ein Paragraph!"}</p>
                </div>
                <div class={classes!("home-section")}>
                    <h3>{"Top 10"}</h3>
                    <p>{"Ein Paragraph!"}</p>
                </div>
                <div class={classes!("home-section")}>
                    <h3>{"News"}</h3>
                    <p>{"Ein Paragraph!"}</p>
                </div>
            </div>
        </>
    }
}
