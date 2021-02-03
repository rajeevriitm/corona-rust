use wasm_bindgen::JsCast;

pub fn initializate_dom() {
    set_canvas_dimension("div-1", "canvas-1");
    set_canvas_dimension("div-2", "canvas-2");
}

fn get_windows() -> web_sys::Window {
    web_sys::window().expect("window dont exist ")
}
fn get_document() -> web_sys::Document {
    get_windows().document().expect("document dont exist")
}
fn get_element_by_id(id: &str) -> web_sys::Element {
    get_document().get_element_by_id(id).unwrap()
}
fn set_canvas_dimension(div_id: &str, canvas_id: &str) {
    let container = get_element_by_id(div_id)
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let width = container.offset_width();
    let height = container.offset_height();
    let canvas = get_element_by_id(canvas_id)
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}
