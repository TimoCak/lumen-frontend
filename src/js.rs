/*
Using this file to translate important methods from JS into Rust
*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
   #[wasm_bindgen(js_namespace = console)]
   pub fn log(s: String);

   #[wasm_bindgen(js_namespace = window)]
   pub fn alert(s: String);

   #[wasm_bindgen(js_namespace = location)]
   pub fn href(s: String);

   #[wasm_bindgen(js_namespace = localStorage)]
   pub fn setItem(key: String, value: JsValue);
   pub fn getItem(key: String);
   pub fn removeItem(key: String);

}