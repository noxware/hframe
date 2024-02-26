use std::any::Any;

/// Keeps track of the state of an HTML Window. Only relevant if you are playing
/// with custom masking strategies (which is considered unstable).
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

    // TODO: Consider making this public, maybe with a different name or from a
    // different place.
    pub(crate) fn get_html_element(&self) -> web_sys::Element {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document.get_element_by_id(&self.id).unwrap()
    }
}
