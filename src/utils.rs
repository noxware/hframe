use crate::mask_strategy::MaskStrategy;
use crate::window_state::WindowState;

macro_rules! eid {
    ($id:expr) => {
        egui::Id::new($id)
    };
}

pub(crate) use eid;

pub(crate) fn rect_to_relative(rect: egui::Rect, parent: egui::Rect) -> egui::Rect {
    let min = rect.min - parent.min;
    let max = rect.max - parent.min;
    egui::Rect::from_min_max(min.to_pos2(), max.to_pos2())
}

pub(crate) fn sync_hframe(state: &WindowState, mask_strategy: &dyn MaskStrategy) {
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

    mask_strategy.mask(&state);
}
