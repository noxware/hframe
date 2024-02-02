use egui::ahash::HashMap;

macro_rules! iframe_style {
    ($rect:expr, $interactable:expr, $visible:expr) => {
        format!(
            "border: none; position: absolute; top: {}px; left: {}px; width: {}px; height: {}px; {}; {}",
            $rect.min.y,
            $rect.min.x,
            $rect.width(),
            $rect.height(),
            if $interactable {
                ""
            } else {
                "pointer-events: none;"
            },
            if $visible {
                ""
            } else {
                "visibility: hidden;"
            }
        )
    };
}

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}
pub struct TemplateApp {
    iframe_handles: IframeHandles,
}

pub struct IframeHandles(HashMap<String, egui::Rect>);

impl IframeHandles {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn create_or_update(
        &mut self,
        id: &str,
        src: &str,
        rect: egui::Rect,
        interactable: bool,
        visible: bool,
    ) {
        self.0.insert(id.to_string(), rect);
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id);

        if let Some(element) = element {
            element
                .set_attribute("style", &iframe_style!(rect, interactable, visible))
                .unwrap();
        } else {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            let iframe = document.create_element("iframe").unwrap();
            iframe.set_attribute("id", id).unwrap();
            iframe.set_attribute("src", src).unwrap();
            iframe
                .set_attribute("style", &iframe_style!(rect, interactable, visible))
                .unwrap();
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
        let shown_window = egui::Window::new("Webview").show(ctx, |ui| {
            //ui.visuals().window_fill;
            ui.centered_and_justified(|ui| {
                ui.label("<web-iframe-content>");
            })
            .response
            .rect
        });

        if let Some(shown_window) = shown_window {
            let interactable = ctx.input(|i| !i.pointer.button_down(egui::PointerButton::Primary));
            // let focused;
            self.iframe_handles.create_or_update(
                "web-iframe-content",
                "https://www.example.org/",
                shown_window.inner.unwrap_or(egui::Rect::ZERO),
                interactable,
                true,
            );
        } else {
            self.iframe_handles.create_or_update(
                "web-iframe-content",
                "https://www.example.org/",
                egui::Rect::ZERO,
                false,
                false,
            );
        }
    }
}
