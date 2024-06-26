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

/// States of a composed area that are tracked by `hframe`.
#[derive(Debug, Clone)]
pub(crate) struct ComposedAreaState {
    /// Normally means the user has the pointer over the area (hover). This is a good opportunity
    /// to enable interactions that may be disabled due to composition reasons.
    pub(crate) is_under_attention: bool,
}

impl ComposedAreaState {
    pub(crate) fn new() -> Self {
        ComposedAreaState {
            is_under_attention: false,
        }
    }
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
    pub(crate) state: ComposedAreaState,
}
