use std::any::Any;

use crate::HtmlWindowState;

/// Metadata used to describe a mask strategy.
#[derive(Clone, Debug)]
pub struct MaskStrategyMeta {
    /// A identifiable name for the strategy.
    pub name: String,
}

/// Trait for implementing custom masking strategies for HTML elements
/// controlled by hframe.
pub trait MaskStrategy: Send + Sync {
    /// Anything that must be done when this strategy is set.
    fn setup(&self);
    /// Anything that must be done when this strategy is unset. For example,
    /// when changing to another strategy at runtime.
    ///
    /// `hframe` already takes care of cleanning inline styles for the HTML
    /// containers of the web content.
    fn cleanup(&self);
    /// A function that can be used to compute any information you may need to
    /// mask the HTML elements later.
    ///
    /// This function has access to all the rects that should be on top of your
    /// element to mask.
    ///
    /// Whatever you return from here will be available at `mask` in
    /// `WindowState`.
    fn compute_mask(
        &self,
        state: &HtmlWindowState,
        prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>>;
    /// A function that will apply the mask to the HTML element referenced by
    /// `state`. From here you can access the data previously computed at
    /// `compute_mask`.
    fn mask(&self, state: &HtmlWindowState);
    /// Return the metadata for this strategy.
    fn meta(&self) -> MaskStrategyMeta;
}
