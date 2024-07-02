use crate::{geo::*, id::*};

/// Holds the HTML data of an HTML area.
#[derive(Debug, Clone)]
pub(crate) struct ComposedHtml {
    /// The id of the HTML element.
    pub(crate) id: String,
    /// The HTML content contained by the area.
    pub(crate) content: String,
}

/// Identifies if this area belongs to the host/HTML world or to the guest/canvas world.
#[derive(Debug, Clone)]
pub(crate) enum ComposedAreaKind {
    Canvas,
    Html(ComposedHtml),
}

/// A rectangular area of the UI that may live in the host/HTML world or in the guest/canvas world.
///
/// This acts as a bridge between the host and guest worlds allowing representing overlapping areas between them or
/// canvas areas that can "hold" HTML areas.
#[derive(Debug, Clone)]
pub(crate) struct ComposedArea {
    pub(crate) id: Id,
    pub(crate) abs_pos: Pos,
    pub(crate) size: Size,
    pub(crate) kind: ComposedAreaKind,
}

impl ComposedArea {
    /// Returns the rectangle that represents this area.
    pub(crate) fn abs_rect(&self) -> Rect {
        Rect::from((self.abs_pos, self.size))
    }
}
