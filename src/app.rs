use std::collections::HashMap;

use serde::Serialize;

/*
<svg viewBox="-10 -10 320 120" xmlns="http://www.w3.org/2000/svg">

  <path
    fill-rule="evenodd"
    stroke="red"
    d="M210,0  h90 v90 h-90 z
           M230,20 v50 h50 v-50 z

           M0,0  h90 v90 h-90 z
           M30,20 v50 h50 v-50 z
           " />
</svg>
*/

/*
<svg width="300" height="300" xmlns="http://www.w3.org/2000/svg">
  <clipPath id="clipRectWithHoles">
    <rect x="0" y="0" width="50" height="300"/>
    <rect x="0" y="0" width="300" height="50"/>
    <rect x="250" y="0" width="50" height="300"/>
    <rect x="0" y="250" width="300" height="50"/>

  </clipPath>

  <!-- Apply the clipping path to an element -->
  <rect width="300" height="300" fill="blue" clip-path="url(#clipRectWithHoles)"/>
</svg>

*/

const TEST_URL: &str = "https://www.example.com/";

macro_rules! iframe_style {
    ($id:expr, $rect:expr, $interactable:expr, $visible:expr) => {
        format!(
            "border: none; position: absolute; top: {}px; left: {}px; width: {}px; height: {}px; {}; {}; clip-path: url(#iframe-clip-{});",
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
            },
            $id
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
    iframes: IframeRegistry,
}

#[derive(Debug, Serialize)]
struct IframeAware {
    rect: egui::Rect,
}

#[derive(Default, Debug, Serialize)]
struct IframeAwares(HashMap<egui::Id, IframeAware>);

impl IframeAwares {
    fn insert<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        let inner_response = inner_response?;
        let response = &inner_response.response;
        self.0.insert(
            response.layer_id.id,
            IframeAware {
                rect: response.rect,
            },
        );
        Some(inner_response)
    }
}

#[derive(Debug, Serialize)]
struct IframeWindowState {
    // All of these should be considered private.
    id: String,
    title: String,
    src: String,
    // Specially the following internal ones.
    open: bool,
    egui_id: egui::Id,
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
            egui_id: egui::Id::new(id),
            open: true,
            interactable: true,
            visible: true,
        }
    }
}

#[derive(Default, Debug, Serialize)]
struct IframeRegistry {
    iframes: Vec<IframeWindowState>,
    iframe_awares: IframeAwares,
}

impl IframeRegistry {
    fn aware<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        self.iframe_awares.insert(inner_response)
    }

    fn insert(&mut self, id: &str, title: &str, src: &str) {
        self.iframes.push(IframeWindowState::new(id, title, src));
    }

    fn show(&mut self, ctx: &egui::Context) {
        for state in &mut self.iframes {
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

            let shown_window = self.iframe_awares.insert(shown_window);

            if let Some(shown_window) = shown_window {
                state.interactable = ctx
                    .input(|i| !i.pointer.button_down(egui::PointerButton::Primary))
                    && ctx.top_layer_id() == Some(shown_window.response.layer_id);
                state.visible = shown_window.inner.is_some();
                state.rect = shown_window.inner.unwrap_or(state.rect);
                sync_iframe(state);
            } else {
                state.visible = false;
                sync_iframe(state);
            }
        }

        self.clip(ctx);
    }

    fn clip(&mut self, ctx: &egui::Context) {
        ctx.memory(|mem| {
            let sorted_awares = mem
                .layer_ids()
                .filter_map(|layer_id| {
                    self.iframe_awares
                        .0
                        .get(&layer_id.id)
                        .map(|aware| (layer_id.id, aware.rect))
                })
                .collect::<Vec<_>>();

            let sorted_awares = sorted_awares.iter().rev().collect::<Vec<_>>();

            let mut clip_pathes = String::new();

            for (index, (id, _rect)) in sorted_awares.iter().enumerate() {
                if let Some(iframe) = self
                    .iframes
                    .iter()
                    .find(|iframe| egui::Id::new(&iframe.id) == *id)
                {
                    let mut path = rect_to_path(&rect_to_relative(iframe.rect, iframe.rect), false);
                    let prev = &sorted_awares[0..index];
                    for (_id, rect) in prev {
                        let relative = rect_to_relative(*rect, iframe.rect);
                        path.push(' ');
                        path.push_str(&rect_to_path(&relative, true));
                    }

                    clip_pathes.push_str(&format!(
                        "<clipPath id=\"iframe-clip-{}\"><path fill-rule=\"evenodd\" d=\"{}\" /></clipPath>",
                        iframe.id, path
                    ));
                }
            }

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let defs = document.get_element_by_id("iframe-clip-defs").unwrap();
            defs.set_inner_html(&clip_pathes);
        });
    }
}

fn rect_to_relative(rect: egui::Rect, parent: egui::Rect) -> egui::Rect {
    let min = rect.min - parent.min;
    let max = rect.max - parent.min;
    egui::Rect::from_min_max(min.to_pos2(), max.to_pos2())
}

fn rect_to_path(rect: &egui::Rect, dt: bool) -> String {
    if !dt {
        format!(
            "M{},{}  h{} v{} h{} z",
            rect.min.x,
            rect.min.y,
            rect.width(),
            rect.height(),
            -rect.width()
        )
    } else {
        format!(
            "M{},{}  v{} h{} v{} z",
            rect.min.x,
            rect.min.y,
            rect.height(),
            rect.width(),
            -rect.height()
        )
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
        self.iframes.aware({
            egui::Window::new("None").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("");
                });
            })
        });

        let devtools = egui::Window::new("Devtools").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("New iframe:");
                ui.text_edit_singleline(&mut self.new_iframe_src);
                if ui.button("Add").clicked() {
                    self.iframe_id_counter += 1;

                    self.iframes.insert(
                        &format!("iframe-{}", self.iframe_id_counter),
                        &format!("Iframe {}", self.iframe_id_counter),
                        &self.new_iframe_src,
                    );
                }
            });
        });

        self.iframes.aware(devtools);
        self.iframes.show(ctx);

        log!("{}", serde_json::to_string_pretty(&self.iframes).unwrap());
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
    let style = iframe_style!(state.id, state.rect, state.interactable, state.visible);
    element.set_attribute("style", &style).unwrap();
}
