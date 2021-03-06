mod model;
mod utils;
use wasm_bindgen::prelude::*;
mod graphics;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, corona-rust!");

    graphics::initializate_dom();
    utils::set_panic_hook();
}
