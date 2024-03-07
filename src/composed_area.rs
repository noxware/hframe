use web_sys::wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ComposedAreaId {
    pub(crate) layer_id: egui::LayerId,
    /// An arbitrary subid to differentiate widgets in the same layer.
    /// This is NULL for windows to differentiate them from widgets inside windows.
    pub(crate) widget_id: egui::Id,
}

impl ComposedAreaId {
    pub(crate) fn new(layer_id: egui::LayerId, widget_id: egui::Id) -> Self {
        Self {
            layer_id,
            widget_id,
        }
    }
}

pub(crate) struct ComposedArea {
    /// An id used to track this area.
    pub(crate) id: ComposedAreaId,
    /// Whole rect of the egui rendered area.
    pub(crate) rect: egui::Rect,
    pub(crate) html: Option<ComposedHtml>,
}

pub(crate) struct ComposedHtml {
    pub(crate) id: String,
    pub(crate) content: String,
    pub(crate) status: ComposedHtmlStatus,
    /// Rect where the HTML content should be placed.
    pub(crate) rect: egui::Rect,
}

impl ComposedHtml {
    pub(crate) fn to_outer_html(&self) -> String {
        let id = &self.id;
        let content = &self.content;
        let styles = self.to_styles();

        format!(r#"<div id="{id}" class="hframe-composed-area" style="{styles}">{content}</div>"#)
    }

    pub(crate) fn to_styles(&self) -> String {
        let top = self.rect.min.y;
        let left = self.rect.min.x;
        let width = self.rect.width();
        let height = self.rect.height();
        let status = self.status.to_styles();

        format!("top: {top}px; left: {left}px; width: {width}px; height: {height}px; {status}")
    }

    pub(crate) fn get_element(&self) -> web_sys::Element {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document
            .get_element_by_id(&self.id)
            .expect("Composed HTML area was not found in the document")
    }

    pub(crate) fn get_html_element(&self) -> web_sys::HtmlElement {
        self.get_element()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
    }
}

pub(crate) struct ComposedHtmlStatus {
    pub(crate) visible: bool,
    pub(crate) interactive: bool,
}

impl ComposedHtmlStatus {
    pub(crate) fn to_styles(&self) -> String {
        let interactive = if self.interactive {
            ""
        } else {
            "pointer-events: none;"
        };
        let visible = if self.visible {
            ""
        } else {
            "visibility: hidden;"
        };

        format!("{interactive} {visible}")
    }
}
