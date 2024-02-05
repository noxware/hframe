use std::collections::{HashMap, HashSet};

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
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />"#;

macro_rules! eid {
    ($id:expr) => {
        egui::Id::new($id)
    };
}

macro_rules! hframe_style {
        ($state:expr) => {
            format!(
                "top: {}px; left: {}px; width: {}px; height: {}px; {}; {}; mask: url({}); -webkit-mask: url({});",
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
struct HframeAware {
    rect: egui::Rect,
}

#[derive(Default, Debug)]
struct HframeAwares(HashMap<egui::Id, HframeAware>);

impl HframeAwares {
    fn insert<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        let inner_response = inner_response?;
        let response = &inner_response.response;
        self.0.insert(
            response.layer_id.id,
            HframeAware {
                rect: response.rect,
            },
        );
        Some(inner_response)
    }
}

#[derive(Debug, Clone)]
struct HframeWindowState {
    // All of these should be considered private.
    id: String,
    title: String,
    content: String,
    // Specially the following internal ones.
    rect: egui::Rect,
    interactable: bool,
    visible: bool,
    mask: String,
    content_changed: bool,
}

impl HframeWindowState {
    fn new(id: &str, title: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            rect: egui::Rect::ZERO,
            interactable: true,
            visible: true,
            mask: build_mask_uri(egui::Rect::ZERO, std::iter::empty()),
            content_changed: false,
        }
    }
}

pub struct Window<'open, 'reg> {
    id: String,
    title: String,
    content: String,
    open: Option<&'open mut bool>,
    registry: &'reg mut Registry,
}

impl<'open, 'reg> Window<'open, 'reg> {
    pub fn open(mut self, open: &'open mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn show(self, ctx: &egui::Context) {
        let Self {
            id,
            title,
            content,
            open,
            registry,
        } = self;

        let open = if let Some(open) = open {
            if !*open {
                return;
            }

            Some(open)
        } else {
            None
        };

        registry.hframes_since_last_sync.insert(id.to_string());

        let state = registry.hframes.iter_mut().find(|state| state.id == id);
        let state = match state {
            Some(state) => state,
            None => {
                registry
                    .hframes
                    .push(HframeWindowState::new(&id, &title, &content));
                registry.hframes.last_mut().unwrap()
            }
        };

        state.content_changed = state.content != content;

        if state.content_changed {
            state.content = content.to_string();
        }

        state.title = title.to_string();

        let window = egui::Window::new(&state.title).id(eid!(&state.id));
        let window = match open {
            Some(open) => window.open(open),
            None => window,
        };

        let shown_window = window.show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("");
            })
            .response
            .rect
        });

        let shown_window = registry.hframe_awares.insert(shown_window);

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
}

#[derive(Debug)]
pub struct Registry {
    hframes: Vec<HframeWindowState>,
    hframe_awares: HframeAwares,
    hframes_since_last_sync: HashSet<String>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let style = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("#hframe-shared-styles")
            .unwrap();

        if style.is_none() {
            let head = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .head()
                .unwrap();

            head.insert_adjacent_html(
                "beforeend",
                &format!(
                    "<style id=\"hframe-shared-styles\">{}</style>",
                    include_str!("hframe.css")
                ),
            )
            .unwrap();
        }

        Self {
            hframes: Vec::new(),
            hframe_awares: HframeAwares::default(),
            hframes_since_last_sync: HashSet::new(),
        }
    }

    pub fn window<'reg>(&'reg mut self, id: &str, title: &str, content: &str) -> Window<'_, 'reg> {
        Window {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            open: None,
            registry: self,
        }
    }

    pub fn aware<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        self.hframe_awares.insert(inner_response)
    }

    fn clip(&mut self, ctx: &egui::Context) {
        ctx.memory(|mem| {
            let sorted_awares = mem
                .layer_ids()
                .filter_map(|layer_id| {
                    self.hframe_awares
                        .0
                        .get(&layer_id.id)
                        .map(|aware| (layer_id.id, aware.rect))
                })
                .collect::<Vec<_>>();

            let sorted_awares = sorted_awares.iter().rev().collect::<Vec<_>>();

            for (index, (id, _rect)) in sorted_awares.iter().enumerate() {
                if let Some(hframe) = self
                    .hframes
                    .iter_mut()
                    .find(|hframe| eid!(&hframe.id) == *id)
                {
                    let prev_rects = sorted_awares[0..index].iter().map(|(_, rect)| *rect);
                    hframe.mask = build_mask_uri(hframe.rect, prev_rects);
                }
            }
        });
    }

    pub fn sync(&mut self, ctx: &egui::Context) {
        self.clip(ctx);
        for state in &self.hframes {
            sync_hframe(state);
        }
        self.clean();
    }

    fn clean(&mut self) {
        self.hframes.retain(|state| {
            if !self.hframes_since_last_sync.contains(&state.id) {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let element = document.get_element_by_id(&state.id).unwrap();
                element.remove();
                self.hframe_awares.0.remove(&eid!(&state.id));
                false
            } else {
                true
            }
        });

        self.hframes_since_last_sync.clear();
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

fn sync_hframe(state: &HframeWindowState) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(&state.id).unwrap_or_else(|| {
        let body = document.body().unwrap();
        let hframe = document.create_element("div").unwrap();
        hframe.set_attribute("id", &state.id).unwrap();
        hframe.set_inner_html(&state.content);
        body.append_child(&hframe).unwrap();
        hframe
    });

    if state.content_changed {
        element.set_inner_html(&state.content);
    }

    let style = hframe_style!(state);
    element.set_attribute("class", "hframe").unwrap();
    element.set_attribute("style", &style).unwrap();
}
