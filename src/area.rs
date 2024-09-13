/// Represents a rectangular entity from the egui canvas or an html element, in unified way.
#[derive(Clone)]
pub(crate) struct Area {
    pub(crate) id: String,
    /// The global x position of the area.
    pub(crate) x: f32,
    /// The global y position of the area.
    pub(crate) y: f32,
    /// The width of the area.
    pub(crate) width: f32,
    /// The height of the area.
    pub(crate) height: f32,
    /// The egui layer this area belongs to, obtained from `layer_id.id`.
    /// An egui area and an html area may share the same id if the first is the parent.
    pub(crate) layer_id: egui::Id,
    /// Is this area just a rectanble in egui/canvas, or it represents an html element to compose?
    /// Html variant contains html specific information.
    pub(crate) kind: AreaKind,
}

#[derive(Clone)]
pub(crate) enum AreaKind {
    Html(AreaHtml),
    Canvas,
}

#[derive(Clone)]
pub(crate) struct AreaHtml {
    /// If html element should exist in the dom but be invisible/minimized.
    pub(crate) visible: bool,
    /// The html content of the area.
    pub(crate) content: String,
}
