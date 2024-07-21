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
        pub(crate) fn log(message: JsValue);
        pub(crate) fn get_pointer_position() -> JsValue;
        pub(crate) async fn sleep_ms(ms: u32);
        pub(crate) async fn transform_element(id: &str, rect: JsValue, holes: JsValue);
        #[wasm_bindgen(catch)]
        pub(crate) fn dangerous_eval(code: &str) -> Result<JsValue, JsValue>;
    }
}

#[wasm_bindgen(main)]
async fn main() {
    console_error_panic_hook::set_once();

    // Draw 3 overlaped rectangles, blue red and green.
    js::dangerous_eval(
        r#"
        const canvasEl = document.getElementById("canvas");
        const ctx = canvasEl.getContext("2d");

        

        ctx.fillStyle = "blue";
        ctx.fillRect(10, 10, 100, 100);

        ctx.fillStyle = "red";
        ctx.fillRect(50, 50, 100, 100);

        ctx.fillStyle = "green";
        ctx.fillRect(90, 90, 100, 100);
        "#,
    )
    .unwrap();

    // Let's try to render this between red and blue.
    js::dangerous_eval(
        r#"
        const el = document.createElement("div");
        el.id = "hframe-yellow";
        el.style.backgroundColor = "yellow";
        el.style.position = "absolute";
        el.style.width = "75px";
        el.style.height = "75px";
        el.style.top = "0";
        el.style.left = "100px";
        document.body.appendChild(el);
    "#,
    )
    .unwrap();

    js::transform_element(
        "hframe-yellow",
        Rect {
            pos: Pos { x: 100.0, y: 0.0 },
            size: Size {
                width: 75.0,
                height: 75.0,
            },
        }
        .to_js_value(),
        vec![Rect {
            pos: Pos { x: 50.0, y: 50.0 },
            size: Size {
                width: 100.0,
                height: 100.0,
            },
        }]
        .to_js_value(),
    )
    .await;

    // loop {
    //     let pos = Pos::from_js_value(js::get_pointer_position());
    //     js::log(pos.to_js_value());
    //     js::sleep_ms(0).await;
    // }
}
