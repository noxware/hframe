use crate::{
    get_composition_context, utils::egui::eid, ComposedArea, ComposedHtml, ComposedHtmlStatus,
};

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

        let open = if let Some(open) = open {
            if !*open {
                return;
            }

            Some(open)
        } else {
            None
        };

        // tel ctx to render html here

        let window = egui::Window::new(title).id(eid!(&id));
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

        if let Some(inner_response) = shown_window {
            let cmp = get_composition_context(ctx);
            let cmp = &mut *cmp.lock().unwrap();
            let ctx = &cmp.egui_ctx;

            let html_visible = inner_response.inner.is_some();
            let html_rect = inner_response.inner.unwrap_or(egui::Rect::ZERO);
            let html_interactive = ctx
                .input(|i| !i.pointer.button_down(egui::PointerButton::Primary))
                && ctx.top_layer_id() == Some(inner_response.response.layer_id);

            cmp.put_composed_area(ComposedArea {
                id: inner_response.response.layer_id.id,
                rect: inner_response.response.rect,
                html: Some(ComposedHtml {
                    id,
                    content,
                    rect: html_rect,
                    status: ComposedHtmlStatus {
                        interactive: html_interactive,
                        visible: html_visible,
                    },
                }),
            })
        }
    }
}
