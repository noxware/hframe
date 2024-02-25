use crate::{get_or_insert_registry, utils::eid, HtmlWindowState};

/// A window capable of displaying HTML content inside.
///
/// It's API mimics egui's Window API.
///
/// Note: `hframe` is automatically aware of this window.
pub struct HtmlWindow<'open> {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) open: Option<&'open mut bool>,
}

impl<'open> HtmlWindow<'open> {
    /// Create a new HtmlWindow.
    ///
    /// This function mimics `new` from egui's Window. It takes the window title
    /// which must be unique as it is used to compute the window id and also to
    /// set ids for HTML elements. Check the `id` method if you want to set a
    /// different id.
    pub fn new(title: &str) -> Self {
        Self {
            id: title.to_lowercase().replace(' ', "-"),
            title: title.to_string(),
            content: "".into(),
            open: None,
        }
    }

    /// Mimics the `open` method of egui's Window.
    pub fn open(mut self, open: &'open mut bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Set a specific id explicitly.
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    /// Set/change the HTML content of the window.
    ///
    /// The initially provided HTML will be used to generete the HTML element.
    /// As long as the HTML doesn't change, this will not re-render the content.
    ///
    /// If you change the content, then the HTML will be re-rendered which is
    /// useful if you need to display controlled and reactive content.
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Displays the window and it's content.
    ///
    /// Note: You will still need to call `sync` at the end of the update loop
    /// to make this work propertly.
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
                    .push(HtmlWindowState::new(&id, &title, &content));
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
