use crate::requests::get_threads::get_posts_by_thread_id;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub thread_id: i32,
}

#[function_component]
pub fn PostListComponent(props: &Props) -> Html {
    let id = props.thread_id.clone();

    let post_list_by_thread_id = use_state(|| vec![]);
    let post_list_by_thread_id_setter = post_list_by_thread_id.setter();

    use_effect_with((), move |_| {
        get_posts_by_thread_id(post_list_by_thread_id_setter, id.clone())
    });

    html! {
        <>
            <link rel={"stylesheet"} href={"/assets/css/post_list_style.css"} />
            {
                post_list_by_thread_id.iter().map(|post| {
                    html!{
                        <div class={"post-list-container"}>
                            <div class={"author"}><a href={""}>{post.author.clone()}</a></div>
                            <div>
                                {post.text.clone()}
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </>
    }
}
