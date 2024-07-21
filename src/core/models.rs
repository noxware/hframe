use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pos {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub(crate) struct Size {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub(crate) struct Rect {
    pub(crate) pos: Pos,
    pub(crate) size: Size,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Area {
    pub(crate) abs_rect: Rect,
    pub(crate) html_content: Option<String>,
    pub(crate) html_id: Option<String>,
}

impl Area {
    pub(crate) fn new(abs_rect: Rect) -> Self {
        Self {
            abs_rect,
            html_content: None,
            html_id: None,
        }
    }

    pub(crate) fn is_html_area(&self) -> bool {
        self.html_id.is_some()
    }

    pub(crate) fn is_canvas_area(&self) -> bool {
        !self.is_html_area()
    }

    pub(crate) fn with_html_content(mut self, content: String) -> Self {
        self.html_content = Some(content);
        self
    }

    pub(crate) fn with_html_id(mut self, id: String) -> Self {
        self.html_id = Some(id);
        self
    }
}
