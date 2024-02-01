pub struct TemplateApp;

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Self coord").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.vertical(|ui| {
                    ui.label(format!("{:?}", ui.next_widget_position()));
                    if ui.button("Open Iframe").clicked() {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let body = document.body().unwrap();
                        let iframe = document.create_element("iframe").unwrap();
                        iframe
                            .set_attribute("src", "https://www.example.com")
                            .unwrap();
                        iframe.set_attribute("width", "100%").unwrap();
                        iframe.set_attribute("height", "100%").unwrap();
                        body.append_child(&iframe).unwrap();
                    }
                });
            });
        });
    }
}
