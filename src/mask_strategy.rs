use std::any::Any;

use crate::window_state::WindowState;

pub trait MaskStrategy {
    fn setup(&self);
    fn cleanup(&self);
    fn compute_mask(
        &self,
        state: &WindowState,
        prev_rects: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any>>;
    fn mask(&self, state: &WindowState);
}
