use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn alert(s: &str);
}

/**
 * A macro that uses Rust's fmt syntax and prints the result to the JavaScript console
 */
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        crate::log(&format_args!($($t)*).to_string())
    };
}

/**
 * The main entry point of the application at this point.
 */
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}
