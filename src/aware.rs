use crate::{area::Area, composition::get_composition};

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
        let cmp = get_composition(egui_ctx);
        let mut cmp = cmp.lock().unwrap();

        cmp.areas.push(Area {
            // id makes sense?
            id: inner_response.response.layer_id.id.short_debug_format(),
            x: inner_response.response.rect.min.x,
            y: inner_response.response.rect.min.y,
            width: inner_response.response.rect.width(),
            height: inner_response.response.rect.height(),
            layer_id: inner_response.response.layer_id.id,
            kind: crate::area::AreaKind::Canvas,
        });

        Some(inner_response)
    }
}
