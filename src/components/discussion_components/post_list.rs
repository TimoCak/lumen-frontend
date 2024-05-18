use crate::apis::backend_api::Backend;
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
        Backend::get_posts_by_thread_id(id.clone(), post_list_by_thread_id_setter)
    });

    html! {
        <>
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
