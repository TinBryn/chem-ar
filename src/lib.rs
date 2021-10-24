use app_state::AppState;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;

mod app_state;
mod objects;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn alert(s: &str);
}

/// A macro that uses Rust's fmt syntax and prints the result to the JavaScript console
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        crate::log(&format_args!($($t)*).to_string())
    };
}

/// The main entry point of the application at this point.
#[wasm_bindgen]
pub fn start() -> Client {
    set_panic_hook();

    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("arDisplay").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    gl.enable(GL::CULL_FACE);
    gl.enable(GL::DEPTH_TEST);

    Client::new(gl)
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGl2RenderingContext,
    cube: objects::Cube,
    app_state: app_state::AppState,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        gl.clear_color(0.7, 0.7, 0.7, 1.0);

        let cube = objects::Cube::new(&gl);
        let app_state = AppState::new();
        Self {
            gl,
            cube,
            app_state,
        }
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.cube
            .render(&self.gl, self.app_state.canvas, self.app_state.angles);
    }

    pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
        self.app_state
            .update(time, app_state::Canvas { width, height });
        self.gl.viewport(0, 0, width as i32, height as i32);
        Ok(())
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
