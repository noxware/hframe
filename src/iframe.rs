use std::collections::HashMap;

const MASK_TEMPLATE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <defs>
    <mask id="mask" x="0" y="0" width="{width}" height="{height}">
      <rect x="0" y="0" width="{width}" height="{height}" fill="white" />
      {holes}      
    </mask>
  </defs>
  <rect x="0" y="0" width="{width}" height="{height}" fill="blue" mask="url(#mask)" />
</svg>
"#;

const HOLE_TEMPLATE: &str =
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" fill="black" />"#;

macro_rules! iframe_style {
        ($state:expr) => {
            format!(
                "border: none; position: absolute; mask-mode: luminance; -webkit-mask-mode: luminance; top: {}px; left: {}px; width: {}px; height: {}px; {}; {}; mask: url({}); -webkit-mask: url({});",
                $state.rect.min.y,
                $state.rect.min.x,
                $state.rect.width(),
                $state.rect.height(),
                if $state.interactable {
                    ""
                } else {
                    "pointer-events: none;"
                },
                if $state.visible {
                    ""
                } else {
                    "visibility: hidden;"
                },
                $state.mask,
                $state.mask
            )
        };
    }

#[derive(Debug)]
struct IframeAware {
    rect: egui::Rect,
}

#[derive(Default, Debug)]
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

#[derive(Debug)]
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
    mask: String,
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
            mask: build_mask_uri(egui::Rect::ZERO, std::iter::empty()),
        }
    }
}

#[derive(Default, Debug)]
pub struct IframeRegistry {
    iframes: Vec<IframeWindowState>,
    iframe_awares: IframeAwares,
}

impl IframeRegistry {
    pub fn aware<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        self.iframe_awares.insert(inner_response)
    }

    pub fn insert(&mut self, id: &str, title: &str, src: &str) {
        self.iframes.push(IframeWindowState::new(id, title, src));
    }

    pub fn show(&mut self, ctx: &egui::Context) {
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
            } else {
                state.visible = false;
            }
        }

        self.clip(ctx);

        for state in &self.iframes {
            sync_iframe(state);
        }
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

            for (index, (id, _rect)) in sorted_awares.iter().enumerate() {
                if let Some(iframe) = self
                    .iframes
                    .iter_mut()
                    .find(|iframe| egui::Id::new(&iframe.id) == *id)
                {
                    let prev_rects = sorted_awares[0..index].iter().map(|(_, rect)| *rect);
                    iframe.mask = build_mask_uri(iframe.rect, prev_rects);
                }
            }
        });
    }
}

fn rect_to_relative(rect: egui::Rect, parent: egui::Rect) -> egui::Rect {
    let min = rect.min - parent.min;
    let max = rect.max - parent.min;
    egui::Rect::from_min_max(min.to_pos2(), max.to_pos2())
}

fn build_mask_uri<H: Iterator<Item = egui::Rect>>(parent: egui::Rect, holes: H) -> String {
    let holes = holes.map(|hole| rect_to_relative(hole, parent));
    let parent = rect_to_relative(parent, parent);

    let holes = holes
        .map(|hole| {
            HOLE_TEMPLATE
                .replace("{x}", &hole.min.x.to_string())
                .replace("{y}", &hole.min.y.to_string())
                .replace("{width}", &hole.width().to_string())
                .replace("{height}", &hole.height().to_string())
        })
        .collect::<String>();

    let svg = MASK_TEMPLATE
        .replace("{width}", &parent.width().to_string())
        .replace("{height}", &parent.height().to_string())
        .replace("{holes}", &holes);

    format!("data:image/svg+xml,{}", urlencoding::encode(&svg))
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
    let style = iframe_style!(state);
    element.set_attribute("style", &style).unwrap();
}
