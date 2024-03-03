use crate::{get_composition_context, ComposedArea};

pub trait Aware {
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
