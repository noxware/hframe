const TEST_URL: &str = "https://www.example.com/";

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

#[allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[derive(Default)]
pub struct TemplateApp {
    iframe_id_counter: usize,
    new_iframe_src: String,
    iframes: Vec<IframeWindowState>,
}

struct IframeWindowState {
    // All of these should be considered private.
    id: String,
    title: String,
    src: String,
    // Specially the following internal ones.
    open: bool,
    rect: egui::Rect,
    interactable: bool,
    visible: bool,
}

impl IframeWindowState {
    fn new(id: &str, title: &str, src: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            src: src.to_string(),
            rect: egui::Rect::ZERO,
            open: true,
            interactable: true,
            visible: true,
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            new_iframe_src: TEST_URL.to_string(),
            ..Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Devtools").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("New iframe:");
                ui.text_edit_singleline(&mut self.new_iframe_src);
                if ui.button("Add").clicked() {
                    self.iframe_id_counter += 1;

                    self.iframes.push(IframeWindowState::new(
                        &format!("iframe-{}", self.iframe_id_counter),
                        &format!("Iframe {}", self.iframe_id_counter),
                        &self.new_iframe_src,
                    ));
                }
            });
        });

        for state in &mut self.iframes {
            show_iframe_window(ctx, state);
        }
    }
}

fn sync_iframe(state: &IframeWindowState) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(&state.id).unwrap_or_else(|| {
        let body = document.body().unwrap();
        let iframe = document.create_element("iframe").unwrap();
        iframe.set_attribute("id", &state.id).unwrap();
        iframe.set_attribute("src", &state.src).unwrap();
        body.append_child(&iframe).unwrap();
        iframe
    });
    let style = iframe_style!(state.rect, state.interactable, state.visible);
    element.set_attribute("style", &style).unwrap();
}

fn show_iframe_window(ctx: &egui::Context, state: &mut IframeWindowState) {
    let shown_window = egui::Window::new(&state.title)
        .id(egui::Id::new(&state.id))
        .open(&mut state.open)
        .show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("");
                // TODO: Display a loader here only when the iframe is actually loading.
            })
            .response
            .rect
        });

    if let Some(shown_window) = shown_window {
        state.interactable = ctx.input(|i| !i.pointer.button_down(egui::PointerButton::Primary));
        state.visible = shown_window.inner.is_some();
        state.rect = shown_window.inner.unwrap_or(state.rect);
        sync_iframe(state);
    } else {
        state.visible = false;
        sync_iframe(state);
    }
}
