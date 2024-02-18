use base64::{engine::general_purpose, Engine as _};
use reqwasm::http::Request;
use web_sys::window;
use yew::{platform::spawn_local, UseForceUpdateHandle, UseStateSetter};
use yew_router::navigator::Navigator;

use crate::{
    BACKEND_URL, FRONTEND_URL,
    models::{
        post::Post,
        thread::{Thread, ThreadForm},
        user::{PostUser, User, UserStored},
    },
    router::Route,
};

pub struct Backend {}

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

    pub fn get_thread_by_id(thread_list: UseStateSetter<Thread>, id: i32) {
        spawn_local(async move {
            let url = format!("{}/threads/{}", BACKEND_URL, id);

            /* need to handle unwraps */
            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Thread>>().await.unwrap();

            thread_list.set(response_text[0].clone());
        });
    }

    pub fn get_posts_by_thread_id(post_list: UseStateSetter<Vec<Post>>, id: i32) {
        spawn_local(async move {
            let url = format!("{}/posts/threads/{}", BACKEND_URL, id);

            let response = Request::get(&url).send().await.unwrap();

            let response_text = response.json::<Vec<Post>>().await.unwrap();

            post_list.set(response_text);
        });
    }

    pub fn post_login(
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
                location.set_href(FRONTEND_URL).unwrap();
            }
        });
    }

    pub fn post_logout(navigator: Navigator, update: UseForceUpdateHandle) {
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

    pub fn post_thread(thread_form: ThreadForm, status_message: UseStateSetter<String>) {
        let storage = window().unwrap().session_storage().unwrap().unwrap();

        spawn_local(async move {
            let url = format!("{}/threads", BACKEND_URL);

            let user_stored: UserStored = serde_json::from_str(
                &storage
                    .get_item("currentUser")
                    .unwrap()
                    .expect("failed to get curerntUser from storage!"),
            )
            .unwrap();

            let auth = general_purpose::STANDARD.encode(format!(
                "{}:{}",
                &user_stored.username, &user_stored.password
            ));
            let response = Request::post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Basic {}", &auth).as_ref())
                .header("withCredentials", "true")
                .body(serde_json::to_string(&thread_form).unwrap())
                .send()
                .await
                .unwrap();

            if response.status() == 201 {
                status_message.set("succesfully posted thread!".to_string());
            } else {
                status_message.set("something went wrong during posting this thread!".to_string());
            }
        });
    }

    pub fn post_user(
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

                use_state_handle_setter.set(fetched_user.clone());

                if response.status() == 200 {
                    navigator.push(&Route::Login);
                }
            });
        }
    }
}
