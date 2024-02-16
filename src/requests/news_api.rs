use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{platform::spawn_local, UseStateSetter};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Source {
    pub id: Option<String>,    
    pub name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct NewsArticle {
    pub source: Source,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub urlToImage: Option<String>,
    pub publishedAt: Option<String>,
    pub content: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct News {
    pub status: String,
    pub totalResults: usize,
    pub articles: Vec<NewsArticle>,
}

impl News {
    fn set_articles(&mut self, articles: Vec<NewsArticle>) {
        self.articles = articles;
    }
}

const NEWS_AMOUNT: usize = 20;

pub fn get_news(news_chunk_setter: UseStateSetter<News>) {
    spawn_local(async move {
        //let url = format!("{}/posts/threads/{}", backend_url, id);
        let url = "https://newsapi.org/v2/everything?q=gaming&language=en&from=2024-02-01&sortBy=publishedAt&apiKey=e7ae4bb45c3f443d8710166599bf1119";        
        let response = Request::get(&url).send().await.unwrap();

        let response_text = response.json::<News>().await;
            
        match response_text {
            Ok(mut v) => {
                v.set_articles(v.articles[1..=NEWS_AMOUNT].to_vec());
                news_chunk_setter.set(v); 
            },
            Err(e) => {
                let e_string = e.to_string();
                let e_js_value: JsValue = JsValue::from_str(&e_string);
                console::log_1(&e_js_value);
            }
        }    
            
         
    });
}