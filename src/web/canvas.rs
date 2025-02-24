//! Interacts with the available in-memory canvas implementation.
//!
//! Should use OffscreenCanvas when available, otherwise use a regular less efficient canvas.
//!
//! TODO: Actually use OffscreenCanvas when available.

use std::cell::{Cell, RefCell};

use super::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

thread_local! {
    static INSTALLED: Cell<bool> = Cell::new(false);
    static CANVAS: RefCell<Option<HtmlCanvasElement>> = RefCell::new(None);
    static CANVAS_CONTEXT: RefCell<Option<CanvasRenderingContext2d>> = RefCell::new(None);
}

/// One time setup of canvas required by further operations.
pub(crate) fn install() {
    let canvas: HtmlCanvasElement = create_element("canvas");
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    CANVAS.set(Some(canvas));
    CANVAS_CONTEXT.set(Some(context));
}

/// Export the image to an URL.
///
/// TODO: A data URL is synchronous and inefficient, use OffscreenCanvas async blob generation
/// instead.
pub(crate) fn export_image() -> String {
    CANVAS.with_borrow(|canvas| {
        let canvas = canvas.as_ref().unwrap();
        let data_url = canvas.to_data_url_with_type("image/png").unwrap();
        data_url
    })
}

/// Set the size of the canvas.
pub(crate) fn set_size(width: u32, height: u32) {
    CANVAS.with_borrow(|canvas| {
        let canvas = canvas.as_ref().unwrap();
        canvas.set_width(width);
        canvas.set_height(height);
    })
}

pub(crate) fn get_size() -> (u32, u32) {
    CANVAS.with_borrow(|canvas| {
        let canvas = canvas.as_ref().unwrap();
        (canvas.width(), canvas.height())
    })
}

/// Clear the whole image buffer of the canvas.
pub(crate) fn clear() {
    let (width, height) = get_size();
    CANVAS_CONTEXT.with_borrow(|context| {
        let context = context.as_ref().unwrap();
        context.clear_rect(0.0, 0.0, width as f64, height as f64);
    })
}

/// Draws a solid rounded rectangle to the canvas.
pub(crate) fn draw_rect(x: f64, y: f64, width: f64, height: f64, radius: f64, fill: &str) {
    CANVAS_CONTEXT.with_borrow(|context| {
        let context = context.as_ref().unwrap();
        context.begin_path();
        context.set_fill_style_str(fill);
        context
            .round_rect_with_f64(x, y, width, height, radius)
            .unwrap();
        context.fill();
    })
}
