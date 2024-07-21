use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/core.js")]
extern "C" {
    pub(crate) fn log(message: JsValue);
    pub(crate) fn get_pointer_position() -> JsValue;
    pub(crate) async fn sleep_ms(ms: u32);
    pub(crate) async fn transform_element(id: &str, rect: JsValue, holes: JsValue);
    pub(crate) fn set_visible(id: &str, visible: bool);
    pub(crate) fn set_pointer_interactivity(id: &str, interactive: bool);
    #[wasm_bindgen(catch)]
    pub(crate) fn dangerous_eval(code: &str) -> Result<JsValue, JsValue>;
}

pub(crate) trait FromJsValue {
    fn from_js_value(value: JsValue) -> Self;
}

pub(crate) trait ToJsValue {
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
