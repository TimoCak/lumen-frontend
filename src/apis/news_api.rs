use chrono::{DateTime, Datelike, Local};
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{platform::spawn_local, UseStateSetter};
use crate::BACKEND_URL;

use crate::models::news::News;

const NEWS_AMOUNT: usize = 20;
const DAY_RANGE: u32 = 7;

const URL: &str = "https://newsapi.org/v2";
const LANGUAGE: &str = "en";
const TOPIC: &str = "gaming";

fn calc_starting_news_date(current_day: u32) -> u32 {
    if current_day > DAY_RANGE {
        return current_day - DAY_RANGE
    }
    1
}

pub fn get_news(news_chunk_setter: UseStateSetter<News>) {
    spawn_local(async move {
        let url = format!("{}/news", BACKEND_URL);

        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<News>().await;

        match response_text {
            Ok(mut v) => {
                if v.articles.len() < NEWS_AMOUNT {
                    v.set_articles(v.articles.to_vec());
                } else {
                    v.set_articles(v.articles[1..=NEWS_AMOUNT].to_vec());
                }
                
                news_chunk_setter.set(v);
            }
            Err(e) => {
                let e_string = e.to_string();
                let e_js_value: JsValue = JsValue::from_str(&e_string);
                console::log_1(&e_js_value);
            }
        }
    });
}
