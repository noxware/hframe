use crate::area::{Area, AreaKind};
use serde::Serialize;

mod js {
    use serde::{de::DeserializeOwned, Serialize};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/companion/dist/companion.js")]
    extern "C" {
        pub(crate) fn set_areas(areas: JsValue);
        pub(crate) fn run();
        pub(crate) fn log(message: JsValue);
    }

    #[allow(dead_code)]
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
}

use js::ToJsValue;

pub(crate) fn install() {
    js::run();
}

pub(crate) fn log(message: &str) {
    js::log(message.to_js_value());
}

#[derive(Serialize)]
struct WebArea {
    id: String,
    kind: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    content: Option<String>,
    interactive: Option<bool>,
    visible: Option<bool>,
}

impl From<Area> for WebArea {
    fn from(area: Area) -> Self {
        let (content, interactive, visible) = match &area.kind {
            AreaKind::Html(html) => (Some(html.content.clone()), Some(false), Some(html.visible)),
            AreaKind::Canvas => (None, None, None),
        };

        let kind = match area.kind {
            AreaKind::Html(_) => "html".to_string(),
            AreaKind::Canvas => "canvas".to_string(),
        };

        Self {
            id: area.id,
            kind,
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
            content,
            interactive,
            visible,
        }
    }
}

pub(crate) fn send_areas(areas: Vec<Area>) {
    let web_areas: Vec<WebArea> = areas.into_iter().map(WebArea::from).collect();
    js::set_areas(web_areas.to_js_value());
}
