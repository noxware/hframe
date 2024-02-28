pub(crate) mod browser_detection;
pub(crate) mod egui;
pub(crate) mod geometry;

use crate::{HtmlWindowState, MaskStrategy};

pub(crate) fn sync_hframe(state: &HtmlWindowState, mask_strategy: &dyn MaskStrategy) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let element = document.get_element_by_id(&state.id).unwrap_or_else(|| {
        let body = document.body().unwrap();
        let hframe = document.create_element("div").unwrap();
        hframe.set_attribute("id", &state.id).unwrap();
        hframe.set_inner_html(&state.content);
        body.append_child(&hframe).unwrap();
        hframe
    });

    if state.content_changed {
        element.set_inner_html(&state.content);
    }

    mask_strategy.mask(state);
}
