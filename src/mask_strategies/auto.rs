use super::DocumentMask;
use crate::{mask_strategy::MaskStrategy, window_state::WindowState};
use std::any::Any;

pub struct Auto {
    inner: Box<dyn MaskStrategy>,
}

impl Auto {
    pub fn new() -> Self {
        Self {
            inner: Box::new(DocumentMask::new()),
        }
    }
}

impl MaskStrategy for Auto {
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
