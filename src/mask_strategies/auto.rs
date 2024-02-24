use super::{DataMask, DocumentMask};
use crate::{
    html_window_state::HtmlWindowState,
    mask_strategy::{MaskStrategy, MaskStrategyMeta},
    utils,
};
use std::any::Any;

pub struct Auto {
    inner: Box<dyn MaskStrategy>,
}

impl Auto {
    pub fn new() -> Self {
        if utils::is_gecko() {
            Self {
                inner: Box::new(DocumentMask::new()),
            }
        } else {
            Self {
                inner: Box::new(DataMask::new()),
            }
        }
    }
}

impl MaskStrategy for Auto {
    fn meta(&self) -> MaskStrategyMeta {
        let mut m = self.inner.meta();
        m.name = format!("{} (auto)", m.name);
        m
    }

    fn setup(&self) {
        self.inner.setup();
    }

    fn cleanup(&self) {
        self.inner.cleanup();
    }

    fn compute_mask(
        &self,
        state: &HtmlWindowState,
        prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
        self.inner.compute_mask(state, prev_rects)
    }

    fn mask(&self, state: &HtmlWindowState) {
        self.inner.mask(state);
    }
}
