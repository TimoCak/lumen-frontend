use reqwasm::http::{Request, Response};

use crate::get_backend_url;
pub struct Http {}

impl Http {
    pub async fn post(url: String, data: String) -> Response {
        let backend_url = get_backend_url();
        let url = format!("{}{}", backend_url, url);
        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .await
            .unwrap();
        response
    }

    pub async fn get(url: String) -> Response {
        let backend_url = get_backend_url();
        let url = format!("{}{}", backend_url, url);
        let response = Request::get(&url).send().await.unwrap();
        response
    }
}
