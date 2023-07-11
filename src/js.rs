/*
Using this file to translate important methods from JS into Rust
*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
   #[wasm_bindgen(js_namespace = console)]
   pub fn log(s: String);

}