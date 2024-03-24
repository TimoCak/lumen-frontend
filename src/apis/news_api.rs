use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{platform::spawn_local, UseStateSetter};
use crate::BACKEND_URL;

use crate::models::news::News;

pub(crate) const NEWS_OFFSET_START: usize = 20;
pub(crate) const NEWS_OFFSET_LIMIT: usize = 100;

pub fn get_news(news_chunk_setter: UseStateSetter<News>, news_amount: usize) {
    spawn_local(async move {
        let url = format!("{}/news", BACKEND_URL);

        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<News>().await;

        match response_text {
            Ok(mut v) => {
                if news_amount <= NEWS_OFFSET_LIMIT {
                    v.set_articles(v.articles[0..news_amount].to_vec());
                    news_chunk_setter.set(v);
                }
            }
            Err(e) => {
                let e_string = e.to_string();
                let e_js_value: JsValue = JsValue::from_str(&e_string);
                console::log_1(&e_js_value);
            }
        }
    });
}
