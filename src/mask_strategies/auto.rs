use super::{DataMask, DocumentMask};
use crate::{
    mask_strategy::{MaskStrategy, MaskStrategyMeta},
    utils,
    window_state::WindowState,
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
        state: &WindowState,
        prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any>> {
        self.inner.compute_mask(state, prev_rects)
    }

    fn mask(&self, state: &WindowState) {
        self.inner.mask(state);
    }
}
