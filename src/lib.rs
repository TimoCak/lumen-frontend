pub mod components;
pub mod style;
pub mod router;
pub mod js;
pub mod requests;

pub fn get_backend_url() -> String {
    String::from("http://localhost:8081/api")
 }

 pub fn get_frontend_url() -> String {
    String::from("http://localhost:8080")
 }