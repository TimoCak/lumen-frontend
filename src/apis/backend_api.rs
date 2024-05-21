use reqwasm::http::Request;
use web_sys::window;
use yew::{platform::spawn_local, Callback, UseForceUpdateHandle, UseStateSetter};
use yew_router::navigator::Navigator;

use crate::{models::{
    post::{Post, PostForm},
    thread::{Thread, ThreadForm},
    user::{PostUser, User, UserStored},
}, router::Route, BACKEND_URL, FRONTEND_URL, get_base64_auth_credentials};

pub struct Backend;

impl Backend {
    pub fn get_threads(thread_list: UseStateSetter<Vec<Thread>>) {
        spawn_local(async move {
            let url = format!("{}/threads", BACKEND_URL);

            /* need to handle unwraps */
            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Thread>>().await.unwrap();

            thread_list.set(response_text);
        });
    }

    pub fn get_thread_by_id(id: i32, thread_list: UseStateSetter<Thread>) {
        spawn_local(async move {
            let url = format!("{}/threads/{}", BACKEND_URL, id);

            /* need to handle unwraps */
            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Thread>>().await.unwrap();

            thread_list.set(response_text[0].clone());
        });
    }

    pub fn create_thread(thread_form: ThreadForm, status_message: UseStateSetter<String>, callback: Callback<Thread>) {
        let url = format!("{}/threads", BACKEND_URL);


        spawn_local(async move {
            let response = Request::post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Basic {}", get_base64_auth_credentials()).as_ref())
                .header("withCredentials", "true")
                .body(&serde_json::to_string(&thread_form).unwrap())
                .send()
                .await
                .unwrap();

            if response.status() == 201 {
                status_message.set("successfully posted thread!".to_string());
                callback.emit(response.json().await.unwrap());
            } else {
                status_message.set("something went wrong during posting this thread!".to_string());
            }
        });
    }

    pub fn update_thread(id: i32, thread: ThreadForm, updated_thread: UseStateSetter<Thread>) {
        spawn_local(async move {
            let url = format!("{}/threads/{}", BACKEND_URL, id);

            let response = Request::put(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&thread).unwrap())
                .send()
                .await
                .unwrap();

            let fetched_thread: Thread = response.json().await.unwrap();
            updated_thread.set(fetched_thread);
        })
    }

    pub fn delete_thread(id: i32) {
        spawn_local(async move {
            let url = format!("{}/threads/{}", BACKEND_URL, id);

            let response = Request::delete(&url)
                .header("Authorization", format!("Basic {}", get_base64_auth_credentials()).as_ref())
                .send()
                .await
                .unwrap();

            //let fetched_thread: Thread = response.json().await.unwrap();
        })
    }

    pub fn get_posts_by_thread_id(id: i32, post_list: UseStateSetter<Vec<Post>>) {
        spawn_local(async move {
            let url = format!("{}/posts/threads/{}", BACKEND_URL, id);

            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Post>>().await.unwrap();

            post_list.set(response_text);
        });
    }

    pub fn get_posts(post_list: UseStateSetter<Vec<Post>>) {
        spawn_local(async move {
            let url = format!("{}/posts", BACKEND_URL);

            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Post>>().await.unwrap();

            post_list.set(response_text);
        });
    }

    pub fn get_post_by_id(id: i32, post_setter: UseStateSetter<Post>) {
        spawn_local(async move {
            let url = format!("{}/posts/{}", BACKEND_URL, id);

            let response = Request::get(&url).send().await.unwrap();

            let fetched_post = response.json::<Post>().await.unwrap();

            post_setter.set(fetched_post);
        })
    }

    pub fn get_posts_by_answer_id(answer_id: i32, post_list: UseStateSetter<Vec<Post>>) {
        spawn_local(async move {
            let url = format!("{}/posts/answers/{}", BACKEND_URL, answer_id);

            let response = Request::get(&url).send().await.unwrap();

            let fetched_post_list = response.json::<Vec<Post>>().await.unwrap();

            post_list.set(fetched_post_list);
        })
    }

    pub fn create_post(post_form: PostForm, created_post: UseStateSetter<Post>) {
        spawn_local(async move {
            let url = format!("{}/posts", BACKEND_URL);

            let response = Request::post(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&post_form).unwrap())
                .send()
                .await
                .unwrap();

            let fetched_post = response.json::<Post>().await.unwrap();

            created_post.set(fetched_post);
        })
    }

    pub fn update_post(id: i32, post: PostForm, updated_post: UseStateSetter<Post>) {
        spawn_local(async move {
            let url = format!("{}/posts/{}", BACKEND_URL, id);

            let response = Request::put(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&post).unwrap())
                .send()
                .await
                .unwrap();

            let fetched_post: Post = response.json().await.unwrap();
            updated_post.set(fetched_post);
        })
    }

    pub fn delete_post(id: i32, deleted_post: UseStateSetter<Post>) {
        spawn_local(async move {
            let url = format!("{}/posts/{}", BACKEND_URL, id);

            let response = Request::delete(&url).send().await.unwrap();

            let fetched_post: Post = response.json().await.unwrap();
            deleted_post.set(fetched_post);
        })
    }

    pub fn sign_in(
        username: &String,
        password: &String,
        navigator: Navigator,
        response_text: UseStateSetter<String>,
    ) {
        let storage = window().unwrap().session_storage().unwrap().unwrap();

        let user = PostUser {
            username: username.to_owned(),
            password: password.to_owned(),
        };

        let location = window().unwrap().location();

        spawn_local(async move {
            let url = format!("{}/sign-in", BACKEND_URL);

            let response = Request::post(&url)
                .header("Content-Type", "application/json")
                .header("Credentials", "includes")
                .body(serde_json::to_string(&user).unwrap())
                .send()
                .await
                .unwrap();

            let response_body = response.text().await.unwrap();

            if response.status() == 400 {
                response_text.set("please fill out all fields!".to_owned());
            } else if response.status() == 401 {
                response_text.set("wrong username or password!".to_owned());
            } else if response.status() == 200 {
                storage
                    .set_item("currentUser", &response_body)
                    .expect("current user set to session storage");
                navigator.push(&Route::Home);
            }
        });
    }

    pub fn sign_out(navigator: Navigator, update: UseForceUpdateHandle) {
        let storage = window().unwrap().session_storage().unwrap().unwrap();

        spawn_local(async move {
            let url = format!("{}/sign-out", BACKEND_URL);

            let response = Request::post(&url).send().await.unwrap();

            if response.status() == 200 {
                storage.clear().unwrap();
                navigator.push(&Route::Home);
                update.force_update();
            }
        });
    }

    pub fn sign_up(
        username: String,
        email: String,
        password: String,
        repeated_password: String,
        use_state_handle_setter: UseStateSetter<String>,
        navigator: Navigator,
    ) {
        let user = User {
            username: username.clone(),
            email: email.clone(),
            password: password.clone(),
        };

        if repeated_password.eq("") || password.eq("") || email.eq("") || username.eq("") {
            use_state_handle_setter.set("fill out all input fields!".to_owned());
        } else if !password.eq(repeated_password.as_str()) {
            use_state_handle_setter.set("Passwords do not match!".to_owned());
        } else {
            spawn_local(async move {
                let data = serde_json::to_string(&user).unwrap();
                let url = format!("{}/sign-up", BACKEND_URL);
                let response = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(data)
                    .send()
                    .await
                    .unwrap();

                let fetched_user = response.text().await.unwrap();

                use_state_handle_setter.set(fetched_user);

                if response.status() == 200 {
                    navigator.push(&Route::Login);
                }
            });
        }
    }

    pub fn get_user(username: String, user_setter: UseStateSetter<User>) {
        spawn_local(async move {
            let url = format!("{}/user/{}", BACKEND_URL, username);
            let response = Request::get(&url).send().await.unwrap();

            let fetched_user: User = response.json().await.unwrap();
            user_setter.set(fetched_user);
        });
    }

    pub fn get_users(users_setter: UseStateSetter<Vec<User>>) {
        spawn_local(async move {
            let url = format!("{}/users", BACKEND_URL);

            let response = Request::get(&url).send().await.unwrap();

            let fetched_users: Vec<User> = response.json().await.unwrap();

            users_setter.set(fetched_users);
        });
    }

    pub fn get_user_by_user_id(id: i32, user_setter: UseStateSetter<User>) {
        spawn_local(async move {
            let url = format!("{}/users/{}", BACKEND_URL, id);
            let response = Request::get(&url).send().await.unwrap();

            let fetched_user: User = response.json().await.unwrap();
            user_setter.set(fetched_user);
        });
    }

    pub fn update_user(id: i32, user: User, updated_user: UseStateSetter<User>) {
        spawn_local(async move {
            let url = format!("{}/users/{}", BACKEND_URL, id);

            let response = Request::put(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&user).unwrap())
                .send()
                .await
                .unwrap();

            let fetched_user: User = response.json().await.unwrap();
            //storing updated user in session storage here
            updated_user.set(fetched_user);
        })
    }

    pub fn delete_user(id: i32, deleted_user: UseStateSetter<UserStored>) {
        spawn_local(async move {
            let url = format!("{}/users/{}", BACKEND_URL, id);

            let response = Request::delete(&url).send().await.unwrap();

            let fetched_user: UserStored = response.json().await.unwrap();
            deleted_user.set(fetched_user)
        })
    }
}
