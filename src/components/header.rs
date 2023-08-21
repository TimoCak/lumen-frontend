use crate::{
    is_user_set, requests::post_logout::post_logout,
    router::Route
};
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

#[function_component]
pub fn HeaderComponent() -> Html {
    let navigator = use_navigator().unwrap();

    let update = use_force_update();

    let logout = Callback::from(move |_| {
        post_logout(navigator.clone(), update.clone());
    });

    html! {
        <>
            <link rel={"stylesheet"} href={"assets/css/header_style.css"}/>
            <div class={classes!("header")}>
                <Link<Route> to={Route::Home} classes={classes!("header-link")}><h1>{"Lumen"}</h1></Link<Route>>
                <Link<Route> to={Route::Showcase} classes={classes!("header-link")}><h2>{"Showcase"}</h2></Link<Route>>
                <Link<Route> to={Route::Discussions} classes={classes!("header-link")}><h2>{"Discussions"}</h2></Link<Route>>

                if is_user_set() {
                    <img onclick={logout} class={classes!("filter-light")} src={"./assets/joystick.svg"}/>
                } else {
                    <Link<Route> to={Route::Login} classes={classes!("header-link")}><img class={classes!("filter-light")} src={"./assets/sign_in.svg"}/></Link<Route>>
                }

            </div>
        </>
    }
}
