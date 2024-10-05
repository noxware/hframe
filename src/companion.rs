use wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component]
fn Companion() -> Html {
    html! {
        <svg style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 1000; pointer-events: none;">
            <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        </svg>
    }
}

pub fn install() {
    let window = web_sys::window();
    let document = window.unwrap().document().unwrap();
    let body = document.body().unwrap();
    let element = document.create_element("div").unwrap();
    let element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
    element.style().set_property("display", "contents").unwrap();
    body.append_child(&element).unwrap();

    yew::Renderer::<Companion>::with_root(element.into()).render();
}
