use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::{HtmlWindowState, MaskStrategy};

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

    mask_strategy.mask(&state);
}

pub(crate) fn is_gecko() -> bool {
    let ua = web_sys::window()
        .unwrap()
        .navigator()
        .user_agent()
        .unwrap()
        .to_lowercase();

    ua.contains("gecko")
        && !ua.contains("like gecko")
        && !ua.contains("webkit")
        && !ua.contains("edge")
        && !ua.contains("trident")
        && !ua.contains("presto")
        && !ua.contains("blink")
        && !ua.contains("chrome")
        && !ua.contains("safari")
        && !ua.contains("opera")
}

pub(crate) struct EguiCheap<T>(Arc<Mutex<T>>);

impl<T> EguiCheap<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }
}

impl<T> Deref for EguiCheap<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> Clone for EguiCheap<T> {
    fn clone(&self) -> Self {
        EguiCheap(self.0.clone())
    }
}
