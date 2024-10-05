use yew::prelude::*;

#[function_component]
fn Companion() -> Html {
    html! {
        <div>
            <h1>{"Companion"}</h1>
            <p>{"This is a companion component."}</p>
        </div>
    }
}

pub fn install() {
    let window = web_sys::window();
    let document = window.unwrap().document().unwrap();
    let body = document.body().unwrap();
    let element = document.create_element("div").unwrap();
    body.append_child(&element).unwrap();

    yew::Renderer::<Companion>::with_root(element).render();
}
