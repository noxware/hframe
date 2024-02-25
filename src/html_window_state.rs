use std::any::Any;

/// Keeps track of the state of an HTML Window. Only relevant if you want to
/// implement your own masking strategy.
#[derive(Debug)]
pub struct HtmlWindowState {
    // All of these should be considered private.
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    // Specially the following internal ones.
    pub rect: egui::Rect,
    pub(crate) interactable: bool,
    pub(crate) visible: bool,
    pub mask: Option<Box<dyn Any + Send>>,
    pub(crate) content_changed: bool,
}

impl HtmlWindowState {
    pub(crate) fn new(id: &str, title: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            rect: egui::Rect::ZERO,
            interactable: true,
            visible: true,
            mask: None,
            content_changed: false,
        }
    }
}
