use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Optimistic getter for the global window object.
pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}

/// Optimistic getter the document in the window.
pub(crate) fn document() -> web_sys::Document {
    window().document().expect("no document")
}

/// Optimistic getter for the body element in the document.
pub(crate) fn body() -> web_sys::HtmlBodyElement {
    document()
        .body()
        .expect("no body element")
        .dyn_into()
        .expect("body element is not a body element")
}

/// Creates an element of the given tag and immediately try to cast it.
///
/// The casted type should be compatible with the tag.
/// For example a `div` could be casted to a `HtmlDivElement`, `HtmlElement`, `Element`, etc.
pub(crate) fn create_element<K: JsCast>(tag: &str) -> K {
    document()
        .create_element(tag)
        .expect("could not create element")
        .dyn_into()
        .expect("could not cast element")
}

/// Set a individual style property in the element.
pub(crate) fn set_style(element: &HtmlElement, key: &str, value: &str) {
    element
        .style()
        .set_property(key, value)
        .expect("could not set style property");
}

/// Shorthand to set multiple style properties in the element.
pub(crate) fn set_styles<'a>(
    element: &HtmlElement,
    styles: impl IntoIterator<Item = (&'a str, &'a str)> + 'a,
) {
    for (key, value) in styles {
        set_style(element, key, value);
    }
}
