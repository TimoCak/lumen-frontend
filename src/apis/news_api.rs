use chrono::{DateTime, Datelike, Local};
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{platform::spawn_local, UseStateSetter};

use crate::models::news::News;

const NEWS_AMOUNT: usize = 20;
const DAY_RANGE: u32 = 7;

const URL: &str = "https://newsapi.org/v2";
const LANGUAGE: &str = "en";
const TOPIC: &str = "gaming";

pub fn get_news(news_chunk_setter: UseStateSetter<News>) {
    spawn_local(async move {
        let local_time: DateTime<Local> = Local::now();

        let datestring = format!(
            "{}-{}-{}",
            local_time.year(),
            local_time.month(),
            local_time.day() - DAY_RANGE
        );

        console::log_1(&JsValue::from_str(&datestring));
        let url = format!("{}/everything?q={}&language={}&from={}&sortBy=publishedAt&apiKey=e7ae4bb45c3f443d8710166599bf1119", URL, TOPIC, LANGUAGE, datestring);

        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<News>().await;

        match response_text {
            Ok(mut v) => {
                v.set_articles(v.articles[1..=NEWS_AMOUNT].to_vec());
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
