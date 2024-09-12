// TODO: CURRENT: Trying to not change the mask until the async mask generation
// is done, and then change the mask in a request anim frame to hook just before
// browser's drawing step.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tap::prelude::*;
use wasm_bindgen::prelude::*;

trait FromJsValue {
    fn from_js_value(value: JsValue) -> Self;
}

trait ToJsValue {
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

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/../companion-js-app/companion.js")]
    extern "C" {
        pub(crate) fn set_areas(areas: JsValue);
        pub(crate) fn run();
    }
}

#[wasm_bindgen(main)]
async fn main() {
    console_error_panic_hook::set_once();
    js::set_areas(serde_wasm_bindgen::to_value(&vec![1, 2, 3]).unwrap());
    js::run();
    js::set_areas(serde_wasm_bindgen::to_value(&vec![1, 2, 3]).unwrap());
}
