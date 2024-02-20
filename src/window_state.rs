use crate::utils;

#[derive(Debug, Clone)]
pub struct WindowState {
    // All of these should be considered private.
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    // Specially the following internal ones.
    pub(crate) rect: egui::Rect,
    pub(crate) interactable: bool,
    pub(crate) visible: bool,
    pub(crate) mask: String,
    pub(crate) content_changed: bool,
}

impl WindowState {
    pub fn new(id: &str, title: &str, content: &str) -> Self {
        let mut res = Self {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            rect: egui::Rect::ZERO,
            interactable: true,
            visible: true,
            mask: "".into(),
            content_changed: false,
        };
        res.mask = utils::build_mask_svg(&res, std::iter::empty());
        res
    }
}
