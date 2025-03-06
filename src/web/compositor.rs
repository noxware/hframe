//! Key module that manipulates web content to compose it with egui content.

use web_sys::HtmlDivElement;

use super::dom::*;
use crate::{
    area::{Area, AreaKind},
    web::canvas,
};
use std::cell::{Cell, RefCell};

thread_local! {
    static INSTALLED: Cell<bool> = Cell::new(false);
    static OVERLAY: RefCell<Option<HtmlDivElement>> = RefCell::new(None);
}

/// One time setup of web content required by further operations.
///
/// This function is idempotent.
pub(crate) fn install() {
    if INSTALLED.get() {
        return;
    } else {
        INSTALLED.set(true);
    }

    canvas::install();

    let overlay: HtmlDivElement = create_element("div");
    set_styles(
        &overlay,
        [
            ("position", "absolute"),
            ("top", "0"),
            ("left", "0"),
            ("width", "100%"),
            ("height", "100%"),
            // Issue: This causes window mousemove events to be ignored.
            ("pointer-events", "none"),
            // TODO: Is this necessary?
            ("z-index", "1000"),
            ("mask", "none"),
        ],
    );
    body().append_child(&overlay).unwrap();
    OVERLAY.set(Some(overlay));
}

fn overlay() -> HtmlDivElement {
    OVERLAY.with_borrow(|overlay| overlay.as_ref().unwrap().clone())
}

/// Updates web content based on the information provided.
///
/// This is a key function for this crate to work, it updates HTML content,
/// clips elements, deals with blending complexities, etc.
pub(crate) fn sync(areas: Vec<Area>) {
    const RADIUS: f64 = 7.0;

    let (width, height) = viewport();
    canvas::set_size(width, height);
    canvas::clear();

    for area in areas {
        let Area {
            x,
            y,
            width,
            height,
            ..
        } = area;

        match area.kind {
            AreaKind::Canvas => {
                canvas::draw_rect(
                    x as f64,
                    y as f64,
                    width as f64,
                    height as f64,
                    RADIUS,
                    "black",
                );
            }
            AreaKind::Html(_) => {
                canvas::draw_rect(
                    x as f64,
                    y as f64,
                    width as f64,
                    height as f64,
                    RADIUS,
                    "white",
                );
            }
        }
    }

    let mask = canvas::export_image();
    // set_style(&overlay(), "mask", &mask);
    set_style(&overlay(), "background-image", &format!("url({})", mask));
}
