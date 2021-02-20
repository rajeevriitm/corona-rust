use super::model;
use wasm_bindgen::JsCast;
pub fn initializate_dom() {
    let window = get_windows();
    let document = get_document(&window);
    // set_canvas_dimension("div-1", "canvas-1", &document);
    set_canvas_dimension("div-2", "canvas-2", &document);
}

fn get_windows() -> web_sys::Window {
    web_sys::window().expect("window dont exist ")
}
fn get_document(window: &web_sys::Window) -> web_sys::Document {
    window.document().expect("document dont exist")
}
fn get_element_by_id(id: &str, document: &web_sys::Document) -> web_sys::Element {
    document.get_element_by_id(id).unwrap()
}
fn set_canvas_dimension(div_id: &str, canvas_id: &str, document: &web_sys::Document) {
    let container = get_element_by_id(div_id, document)
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let width = container.offset_width();
    let height = container.offset_height();
    // web_sys::console::log_1(&width.into());

    let canvas = get_element_by_id(canvas_id, document)
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}
