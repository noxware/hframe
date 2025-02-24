//! Key module that manipulates web content to compose it with egui content.

use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use super::{canvas, dom::*};
use crate::area::Area;
use std::{
    cell::{Cell, RefCell},
    sync::{Once, OnceLock},
    thread,
};

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
        ],
    );
    body().append_child(&overlay).unwrap();
    OVERLAY.set(Some(overlay));
}

/// Updates web content based on the information provided.
///
/// This is a key function for this crate to work, it updates HTML content,
/// clips elements, deals with blending complexities, etc.
pub(crate) fn sync(areas: Vec<Area>) {}
