use std::any::Any;

use crate::HtmlWindowState;

#[derive(Clone, Debug)]
pub struct MaskStrategyMeta {
    pub name: String,
}

pub trait MaskStrategy: Send + Sync {
    fn setup(&self);
    fn cleanup(&self);
    fn compute_mask(
        &self,
        state: &HtmlWindowState,
        prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>>;
    fn mask(&self, state: &HtmlWindowState);
    fn meta(&self) -> MaskStrategyMeta;
}
