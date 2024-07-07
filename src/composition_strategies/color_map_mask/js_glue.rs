use super::helpers::Color;
use wasm_bindgen::prelude::*;

pub fn eval_js(js: &str) -> Result<JsValue, JsValue> {
    js_sys::eval(js)
}

pub fn clear() {
    eval_js("window.color_map_mask.actions.clear();").unwrap();
}

pub fn setup() {
    eval_js(include_str!("companion.js")).unwrap();
}

pub fn draw_rect(x: f64, y: f64, width: f64, height: f64, color: &str) {
    let js = format!(
        "window.color_map_mask.actions.drawRect({}, {}, {}, {}, '{}');",
        x, y, width, height, color
    );
    eval_js(&js).unwrap();
}

pub fn get_pixel_color(x: f64, y: f64) -> Color {
    let js = format!(
        "window.color_map_mask.result = window.color_map_mask.actions.getPixelColor({}, {});",
        x, y
    );
    eval_js(&js).unwrap();

    let r = eval_js("window.color_map_mask.result.r")
        .unwrap()
        .as_f64()
        .unwrap() as u8;
    let g = eval_js("window.color_map_mask.result.g")
        .unwrap()
        .as_f64()
        .unwrap() as u8;
    let b = eval_js("window.color_map_mask.result.b")
        .unwrap()
        .as_f64()
        .unwrap() as u8;
    let a = eval_js("window.color_map_mask.result.a")
        .unwrap()
        .as_f64()
        .unwrap() as u8;

    Color::from((r, g, b, a))
}
