use crate::{
    get_composition_context, utils::egui::eid, ComposedArea, ComposedAreaId, ComposedHtml,
    ComposedHtmlStatus,
};

/// A widget to display HTML content anywhere, without the need of a window.
///
/// Similar to HtmlWindow, if this builder is not called in the current update call,
/// the associated HTML content will be destroyed. But HtmlWindow supports window minimization
/// without this limitation.
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

        let egui_ctx = ui.ctx();

        // TODO: DRY this.
        let interactive = egui_ctx.input(|i| !i.pointer.button_down(egui::PointerButton::Primary))
            && egui_ctx.top_layer_id() == Some(response.layer_id);

        let cmp = get_composition_context(ui.ctx());
        let cmp = &mut *cmp.lock().unwrap();

        cmp.put_composed_area(ComposedArea {
            id: ComposedAreaId::new(response.layer_id, eid!(&self.id)),
            rect: response.rect,
            html: Some(ComposedHtml {
                id: self.id.clone(),
                content: self.content.clone(),
                rect: response.rect,
                status: ComposedHtmlStatus {
                    interactive,
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
