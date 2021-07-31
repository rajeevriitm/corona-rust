const SLING_SPEED: u32 = 2;
const SLING_TOP_PADDING: u32 = 5;
const SLING_BOTTOM_PADDING: u32 = 95;
const SLING_MIN_ANGLE: i32 = -60;
const SLING_MAX_ANGLE: i32 = 60;
const SLING_ROTATE_SPEED: i32 = 5;
// pub struct World {
//     pub canvas: web_sys::HtmlCanvasElement,
// }
// impl World {
//     fn new(canvas: web_sys::HtmlCanvasElement) -> World {
//         World { canvas }
//     }
// }
pub struct Sling {
    pub top: u32,
    pub angle: i32,
    pub html_element: web_sys::HtmlElement,
}
impl Sling {
    pub fn new(top: u32, angle: i32, html_element: web_sys::HtmlElement) -> Sling {
        Sling {
            top,
            angle,
            html_element,
        }
    }
    pub fn move_sling_down(&mut self) {
        let val = std::cmp::min(SLING_BOTTOM_PADDING, self.top + SLING_SPEED);
        self.top = val;
    }
    pub fn move_sling_up(&mut self) {
        let val = std::cmp::max(SLING_TOP_PADDING, self.top - SLING_SPEED);
        self.top = val;
    }
    pub fn rotate_sling_clock(&mut self) {
        let val = std::cmp::min(SLING_MAX_ANGLE, self.angle + SLING_ROTATE_SPEED);
        self.angle = val;
    }
    pub fn rotate_sling_anticlock(&mut self) {
        let val = std::cmp::max(SLING_MIN_ANGLE, self.angle - SLING_ROTATE_SPEED);
        self.angle = val;
    }
}
pub struct Point {
    x: u32,
    y: u32,
}
pub enum GameState {
    Aiming,
    Shot,
}
pub struct World {
    height: u32,
    width: u32,
}
impl World {
    fn new(width: u32, height: u32) -> World {
        World { height, width }
    }
}
