use crate::{get_composition_context, ComposedArea};

/// Allows you to implement `aware` for egui entities so hframe can know about
/// their existence when applying compositions.
pub trait Aware {
    /// Let hframe know about the existence of this entity.
    ///
    /// You must call this in anything from the egui world that can overlap
    /// with HTML content (like normal egui windows).
    fn aware(self) -> Self;
}

impl<R> Aware for Option<egui::InnerResponse<R>> {
    fn aware(self) -> Self {
        let inner_response = self?;
        let egui_ctx = &inner_response.response.ctx;
        let cmp = get_composition_context(egui_ctx);
        let mut cmp = cmp.lock().unwrap();

        cmp.put_composed_area(ComposedArea {
            id: inner_response.response.layer_id.id,
            rect: inner_response.response.rect,
            html: None,
        });

        Some(inner_response)
    }
}
