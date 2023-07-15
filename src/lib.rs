pub mod components;
pub mod style;
pub mod router;
pub mod js;

pub fn get_backend_url() -> String {
    String::from("http://localhost:8081/api")
 }