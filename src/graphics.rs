use super::model::Sling;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub fn initializate_dom() {
    let window = get_windows();
    let document = get_document(&window);
    // set_canvas_dimension("div-1", "canvas-1", &document);
    set_canvas_dimension("div-2", "canvas-2", &document);
    let sling_div = get_element_by_id("div-1", &document);
    // .dyn_into::<web_sys::HtmlElement>()
    // .unwrap();
    // let sling_div1 = get_element_by_id("div-1", &document)
    //     .dyn_into::<web_sys::HtmlElement>()
    //     .unwrap();
    let sling_div_height = sling_div.client_height();
    // lazy_static! {
    //     static ref
    // }
    // let sling_div_height1 = sling_div1.offset_height();
    let sling_img = get_element_by_id("sling-img", &document)
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let sling_img_top = sling_img.style();

    // .get_property_value("width").unwrap();
    log(&sling_img_top);
    let mut sling = Sling::new(40, -20, sling_img);
    draw_sling(&sling);
    let keypress_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        match key.as_str() {
            "ArrowUp" => {
                sling.move_sling_up();
            }
            "ArrowDown" => {
                sling.move_sling_down();
            }
            "ArrowRight" => {
                sling.rotate_sling_anticlock();
            }
            "ArrowLeft" => {
                sling.rotate_sling_clock();
            }
            _ => {}
        }
        draw_sling(&sling);
    }) as Box<dyn FnMut(_)>);
    keydown_event(&document, &keypress_closure);
    keypress_closure.forget();
}
fn draw_sling(sling: &Sling) {
    let style = sling.html_element.style();

    style
        .set_property("top", &format!("{}%", sling.top))
        .unwrap();
    style
        .set_property("transform", &format!("rotate({}deg)", sling.angle))
        .unwrap();
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
fn keydown_event(
    document: &web_sys::Document,
    closure: &Closure<dyn FnMut(web_sys::KeyboardEvent)>,
) {
    document.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
}
fn log<T: Into<JsValue>>(value: T) {
    web_sys::console::log_1(&value.into())
}
