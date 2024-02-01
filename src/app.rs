use egui::ahash::HashMap;

macro_rules! iframe_style {
    ($rect:expr) => {
        format!(
            "position: absolute; top: {}px; left: {}px; width: {}px; height: {}px; border: none; pointer-events: none;",
            $rect.min.y,
            $rect.min.x,
            $rect.width(),
            $rect.height()
        )
    };
}

pub struct TemplateApp {
    iframe_handles: IframeHandles,
}

pub struct IframeHandles(HashMap<String, egui::Rect>);

impl IframeHandles {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn create_or_update(&mut self, id: &str, url: &str, rect: egui::Rect) {
        self.0.insert(id.to_string(), rect);
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id);

        if let Some(element) = element {
            element
                .set_attribute("style", &iframe_style!(rect))
                .unwrap();
        } else {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            let iframe = document.create_element("iframe").unwrap();
            iframe.set_attribute("id", id).unwrap();
            iframe.set_attribute("src", url).unwrap();
            iframe.set_attribute("style", &iframe_style!(rect)).unwrap();
            body.append_child(&iframe).unwrap();
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            iframe_handles: IframeHandles::new(),
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Webview").show(ctx, |ui| {
            let container = ui
                .centered_and_justified(|ui| {
                    ui.label("<web-iframe-content>");
                })
                .response;
            self.iframe_handles.create_or_update(
                "web-iframe-content",
                "https://www.example.com",
                container.rect,
            );
        });
    }
}
