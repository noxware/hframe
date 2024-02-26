use crate::{
    html_window_state::HtmlWindowState,
    mask_strategy::{MaskStrategy, MaskStrategyMeta},
};
use std::any::Any;

/// A non-op strategy that does nothing.
///
/// Using this disables masking.
pub struct Nop;

impl Nop {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl MaskStrategy for Nop {
    fn meta(&self) -> MaskStrategyMeta {
        MaskStrategyMeta { name: "nop".into() }
    }

    fn setup(&self) {}

    fn cleanup(&self) {}

    fn compute_mask(
        &self,
        _state: &HtmlWindowState,
        _prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
        None
    }

    fn mask(&self, _state: &HtmlWindowState) {}
}
