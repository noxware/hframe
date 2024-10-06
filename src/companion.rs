use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    SinkExt, StreamExt,
};
use std::sync::{Mutex, OnceLock};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::area::Area;

// Most yew types are not `Send` what is going to cause problems later when integrating with egui.
// Also, the receiver can't be passed as props to `App` as it doesn't implement `PartialEq` and I
// don't want to give it a random one.
static RX: OnceLock<Mutex<UnboundedReceiver<Vec<Area>>>> = OnceLock::new();

// Just to be consitent with RX :P Not much of a harm.
static TX: OnceLock<Mutex<UnboundedSender<Vec<Area>>>> = OnceLock::new();

#[function_component]
fn App() -> Html {
    let areas = use_state(|| Vec::<Area>::new());

    let areas_clone = areas.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(areas) = RX.get().unwrap().lock().unwrap().next().await {
                crate::web::log("updating");
                areas_clone.set(areas);
            }
        });
    });

    html! {
        <>
            <svg style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 1000; pointer-events: none;">
                <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
            </svg>
            <p>{format!("{:?}", areas)}</p>
        </>
    }
}

pub(crate) fn install() {
    let window = web_sys::window();
    let document = window.unwrap().document().unwrap();
    let body = document.body().unwrap();
    let element = document.create_element("div").unwrap();
    let element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
    element.style().set_property("display", "contents").unwrap();
    body.append_child(&element).unwrap();

    let (tx, rx) = futures::channel::mpsc::unbounded::<Vec<Area>>();
    TX.set(Mutex::new(tx)).unwrap();
    RX.set(Mutex::new(rx)).unwrap();

    yew::Renderer::<App>::with_root(element.into()).render();
}

pub(crate) fn send_areas(areas: Vec<Area>) {
    wasm_bindgen_futures::spawn_local(async {
        TX.get().unwrap().lock().unwrap().send(areas).await.unwrap();
    });
}
