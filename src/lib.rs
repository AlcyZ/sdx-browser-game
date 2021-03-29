use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(msg: &str);
}

#[wasm_bindgen]
pub fn test(msg: &str) {
    let message = format!("wasm: {}", msg);
    alert(&message);
    web_sys::console::log_1(&message.into());
}

