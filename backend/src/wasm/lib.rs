use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_input(input: &str) -> String {
    format!("AI processed: {}", input)
}