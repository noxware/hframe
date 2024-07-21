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
        pub(crate) fn render_fake_widget(widget: JsValue);
        pub(crate) fn clear_fake_widgets();
    }
}

mod ui {
    use super::*;

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

    /// Simulate a UI element in a GUI lib like egui.
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub(crate) struct FakeWidget {
        pub(crate) color: String,
        pub(crate) area: Area,
    }

    impl FakeWidget {
        pub(crate) fn new(color: String, area: Area) -> Self {
            Self { color, area }
        }

        pub(crate) fn render(&self) {
            js::render_fake_widget(self.to_js_value());
        }
    }
}

use ui::*;

#[wasm_bindgen(main)]
async fn main() {
    console_error_panic_hook::set_once();

    let mut widgets = vec![
        ui::FakeWidget::new(
            "blue".to_string(),
            Area::new(Rect {
                pos: Pos { x: 10.0, y: 10.0 },
                size: Size {
                    width: 100.0,
                    height: 100.0,
                },
            }),
        ),
        ui::FakeWidget::new(
            "yellow".to_string(),
            Area::new(Rect {
                pos: Pos { x: 100.0, y: 0.0 },
                size: Size {
                    width: 75.0,
                    height: 75.0,
                },
            })
            .with_html_id("hframe-yellow".to_string())
            .with_html_content("hello".to_string()),
        ),
        ui::FakeWidget::new(
            "red".to_string(),
            Area::new(Rect {
                pos: Pos { x: 50.0, y: 50.0 },
                size: Size {
                    width: 100.0,
                    height: 100.0,
                },
            }),
        ),
        ui::FakeWidget::new(
            "green".to_string(),
            Area::new(Rect {
                pos: Pos { x: 90.0, y: 90.0 },
                size: Size {
                    width: 100.0,
                    height: 100.0,
                },
            }),
        ),
    ];

    loop {
        js::clear_fake_widgets();
        for widget in &widgets {
            widget.render();
        }

        js::transform_element(
            &widgets[1].area.html_id.as_ref().unwrap(),
            widgets[1].area.abs_rect.to_js_value(),
            vec![widgets[0].area.abs_rect].to_js_value(),
        )
        .await;

        widgets[1].area.abs_rect.pos.x += 1.0;
        widgets[0].area.abs_rect.pos.x += 2.0;

        js::sleep_ms(200).await;
    }
}
