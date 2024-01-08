mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn hell_world(s: &str) {
    console::log_2(&"hello_world:".into(), &"rust".into())
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}