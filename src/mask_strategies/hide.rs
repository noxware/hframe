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
            if $state.interactable && $state.mask.is_none() {
                ""
            } else {
                "pointer-events: none;"
            },
            if $state.visible && $state.mask.is_none() {
                ""
            } else {
                "visibility: hidden;"
            },
        )
    };
}

struct Empty;

/// A strategy that hides the HTML element if an aware area is in front of it.
///
/// This is simple, well-supported and performant but you lose the ability to
/// put stuff in front of your HTML content.
pub struct Hide;

impl Hide {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl MaskStrategy for Hide {
    fn meta(&self) -> MaskStrategyMeta {
        MaskStrategyMeta {
            name: "hide".into(),
        }
    }

    fn setup(&self) {}

    fn cleanup(&self) {}

    fn compute_mask(
        &self,
        _ctx: &egui::Context,
        _hframe: &HtmlWindowState,
        holes: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
        match holes.next() {
            Some(_) => Some(Box::new(Empty)),
            None => None,
        }
    }

    fn mask(&self, state: &HtmlWindowState) {
        let element = state.get_html_element();
        let style = hframe_style!(state);
        element.set_attribute("class", "hframe").unwrap();
        element.set_attribute("style", &style).unwrap();
    }
}
