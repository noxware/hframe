use crate::{
    get_composition_context, utils::egui::eid, ComposedArea, ComposedAreaId, ComposedHtml,
    ComposedHtmlStatus,
};

/// A widget to display HTML content OVER any area.
///
/// This does NOT perform automatic composition, it just "floats" over the canvas, but
/// follows the area where it was placed.
///
/// This is more than enough if your HTML content will not overlap with other content,
/// for example, in side panels, full screen overlays, etc.
///  
/// Note: `hframe` is automatically aware of this widget.
pub struct BareHtmlOverlay {
    pub(crate) id: String,
    pub(crate) content: String,
}

impl BareHtmlOverlay {
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

impl egui::Widget for &mut BareHtmlOverlay {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui
            .centered_and_justified(|ui| {
                ui.label("");
            })
            .response;

        let cmp = get_composition_context(ui.ctx());
        let cmp = &mut *cmp.lock().unwrap();

        cmp.put_composed_area(ComposedArea {
            id: ComposedAreaId::new(egui::LayerId::background(), eid!(&self.id)),
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

impl egui::Widget for BareHtmlOverlay {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut this = self;
        (&mut this).ui(ui)
    }
}
