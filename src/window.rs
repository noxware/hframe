use crate::{get_or_insert_registry, utils::eid, WindowState};

pub struct Window<'open> {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) open: Option<&'open mut bool>,
}

impl<'open> Window<'open> {
    pub fn new(title: &str) -> Self {
        Self {
            id: title.to_lowercase().replace(' ', "-"),
            title: title.to_string(),
            content: "".into(),
            open: None,
        }
    }

    pub fn open(mut self, open: &'open mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn show(self, ctx: &egui::Context) {
        let Self {
            id,
            title,
            content,
            open,
        } = self;

        let registry = get_or_insert_registry(ctx);
        let registry = &mut *registry.lock().unwrap();

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
                    .push(WindowState::new(&id, &title, &content));
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
