use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Size {
    width: f64,
    height: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Rect {
    pos: Pos,
    size: Size,
}

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

    #[wasm_bindgen(module = "/lib.js")]
    extern "C" {
        #[wasm_bindgen]
        pub(crate) fn log(message: JsValue);
        #[wasm_bindgen(js_name = getPointerPosition)]
        pub(crate) fn get_pointer_position() -> JsValue;
        #[wasm_bindgen(js_name = sleepMs)]
        pub(crate) async fn sleep_ms(ms: u32);

    }
}

#[wasm_bindgen(main)]
async fn main() {
    console_error_panic_hook::set_once();

    loop {
        let pos = Pos::from_js_value(js::get_pointer_position());
        js::log(pos.to_js_value());
        js::sleep_ms(0).await;
    }
}
