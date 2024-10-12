use crate::{
    area::{Area, AreaHtml, AreaKind},
    composition::get_composition,
};

pub struct HtmlFrame {
    pub(crate) id: String,
    pub(crate) content: String,
}

impl HtmlFrame {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            content: "".into(),
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
}

impl egui::Widget for &mut HtmlFrame {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Just use something that takes the whole space.
        let response = ui
            .centered_and_justified(|ui| {
                ui.label("");
            })
            .response;

        let html_rect = response.rect;
        let html_x = html_rect.min.x;
        let html_y = html_rect.min.y;
        let html_width = html_rect.width();
        let html_height = html_rect.height();
        let layer_id = response.layer_id.id;

        let cmp = get_composition(ui.ctx());
        let cmp = &mut *cmp.lock().unwrap();

        // Note: Consider pushing a canvas area here as well below the html area.
        // Maybe is unnecessary.

        cmp.areas.push(Area {
            id: format!("{}_html", self.id),
            x: html_x,
            y: html_y,
            width: html_width,
            height: html_height,
            layer_id,
            kind: AreaKind::Html(AreaHtml {
                visible: true, // for now, always visible
                content: self.content.clone(),
            }),
        });

        response
    }
}

impl egui::Widget for HtmlFrame {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        (&mut self).ui(ui)
    }
}
