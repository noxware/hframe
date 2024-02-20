use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Aware {
    pub(crate) rect: egui::Rect,
}

#[derive(Default, Debug)]
pub(crate) struct Awares(pub(crate) HashMap<egui::Id, Aware>);

impl Awares {
    pub(crate) fn insert<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        let inner_response = inner_response?;
        let response = &inner_response.response;
        self.0.insert(
            response.layer_id.id,
            Aware {
                rect: response.rect,
            },
        );
        Some(inner_response)
    }
}
