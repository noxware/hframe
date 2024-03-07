use crate::{
    get_composition_context, utils::egui::eid, ComposedArea, ComposedHtml, ComposedHtmlStatus,
};

/// A widget to display HTML content anywhere, without the need of a window.
///
/// Note: `hframe` is automatically aware of this widget.
pub struct BareHtml {
    pub(crate) id: String,
    pub(crate) content: String,
}

impl BareHtml {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_lowercase().replace(' ', "-"),
            content: "".into(),
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
}

impl egui::Widget for &mut BareHtml {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui
            .centered_and_justified(|ui| {
                ui.label("");
            })
            .response;

        let cmp = get_composition_context(&ui.ctx());
        let cmp = &mut *cmp.lock().unwrap();

        web_sys::console::debug_2(
            &web_sys::wasm_bindgen::JsValue::from("> bare layer id"),
            &web_sys::wasm_bindgen::JsValue::from(&response.layer_id.id.short_debug_format()),
        );

        cmp.put_composed_area(ComposedArea {
            // This doesn't work because is the id of the window which was also
            // awared as a non HTML area.
            id: response.layer_id.id,
            rect: response.rect,
            html: Some(ComposedHtml {
                id: self.id.clone(),
                content: self.content.clone(),
                rect: response.rect,
                status: ComposedHtmlStatus {
                    interactive: true,
                    visible: true,
                },
            }),
        });

        response
    }
}

impl egui::Widget for BareHtml {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut this = self;
        (&mut this).ui(ui)
    }
}
