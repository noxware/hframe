use std::any::Any;

#[derive(Debug)]
pub struct WindowState {
    // All of these should be considered private.
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    // Specially the following internal ones.
    pub(crate) rect: egui::Rect,
    pub(crate) interactable: bool,
    pub(crate) visible: bool,
    pub(crate) mask: Option<Box<dyn Any>>,
    pub(crate) content_changed: bool,
}

impl WindowState {
    pub fn new(id: &str, title: &str, content: &str) -> Self {
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
