use crate::{registry::Registry, utils::eid, window_state::WindowState};

pub struct Window<'open, 'reg> {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) open: Option<&'open mut bool>,
    pub(crate) registry: &'reg mut Registry,
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
