use crate::{geo::*, id::*};

#[derive(Debug, Clone)]
pub(crate) struct ComposedHtml {
    pub(crate) content: String,
}

#[derive(Debug, Clone)]
pub(crate) enum ComposedAreaKind {
    Canvas,
    Html(ComposedHtml),
}

#[derive(Debug, Clone)]
pub(crate) struct ComposedAreaState {
    pub(crate) active: bool,
}

impl ComposedAreaState {
    pub(crate) fn new() -> Self {
        ComposedAreaState { active: false }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ComposedArea {
    pub(crate) id: Id,
    pub(crate) abs_pos: Pos,
    pub(crate) size: Size,
    pub(crate) kind: ComposedAreaKind,
    pub(crate) state: ComposedAreaState,
}
