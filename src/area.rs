pub(crate) struct Area {
    pub(crate) id: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) kind: AreaKind,
}

pub(crate) enum AreaKind {
    Html(AreaHtml),
    Canvas,
}

pub(crate) struct AreaHtml {
    pub(crate) visible: bool,
    pub(crate) content: String,
}
