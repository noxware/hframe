use crate::{
    html_window_state::HtmlWindowState,
    mask_strategy::{MaskStrategy, MaskStrategyMeta},
};
use std::any::Any;

macro_rules! hframe_style {
    ($state:expr) => {
        format!(
            "top: {}px; left: {}px; width: {}px; height: {}px; {}; {};",
            $state.rect.min.y,
            $state.rect.min.x,
            $state.rect.width(),
            $state.rect.height(),
            if $state.interactable {
                ""
            } else {
                "pointer-events: none;"
            },
            if $state.visible {
                ""
            } else {
                "visibility: hidden;"
            },
        )
    };
}

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
        _ctx: &egui::Context,
        _state: &HtmlWindowState,
        _prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
        None
    }

    fn mask(&self, state: &HtmlWindowState) {
        let element = state.get_html_element();
        let style = hframe_style!(state);
        element.set_attribute("class", "hframe").unwrap();
        element.set_attribute("style", &style).unwrap();
    }
}
