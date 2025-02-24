//! Contains utilities to deal with JS.

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

/// Expose JS log function.
pub(crate) fn log(data: &JsValue) {
    web_sys::console::log_1(data);
}

pub(crate) trait FromJsValue {
    /// Convert to this Rust type from a JS value.
    fn from_js_value(value: JsValue) -> Self;
}

pub(crate) trait ToJsValue {
    /// Convert to a JS value from this Rust type.
    fn to_js_value(&self) -> JsValue;
}

impl<T> FromJsValue for T
where
    T: DeserializeOwned,
{
    fn from_js_value(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}

impl<T> ToJsValue for T
where
    T: Serialize,
{
    fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}
