use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn synthesize(examples: Vec<String>) -> String {
    if let Some(first) = examples.first() {
        infer_pattern(first)
    } else {
        ".*".to_string()
    }
}

fn infer_pattern(text: &str) -> String {
    if text.chars().all(|c| c.is_ascii_alphabetic()) {
        "[a-zA-Z]+".to_string()
    } else if text.chars().all(|c| c.is_ascii_digit()) {
        "\\d+".to_string()
    } else {
        ".*".to_string()
    }
}
