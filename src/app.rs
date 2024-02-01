pub struct TemplateApp;

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Self coord").show(ctx, |ui| {
            ui.vertical(|ui| {
                let rect = ui.label("ref").rect;
                ui.label(format!("{:?}", rect));
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
    }
}
