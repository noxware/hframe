use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    SinkExt, StreamExt,
};
use std::{
    ops::Deref,
    sync::{Mutex, OnceLock},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::area::{Area, AreaKind};

// Most yew types are not `Send` what is going to cause problems later when integrating with egui.
// Also, the receiver can't be passed as props to `App` as it doesn't implement `PartialEq` and I
// don't want to give it a random one.
static RX: OnceLock<Mutex<UnboundedReceiver<Vec<Area>>>> = OnceLock::new();

// Just to be consitent with RX :P Not much of a harm.
static TX: OnceLock<Mutex<UnboundedSender<Vec<Area>>>> = OnceLock::new();

#[derive(Properties, PartialEq)]
struct AreaProps {
    area: Area,
    interactive: bool,
}

#[function_component]
fn EmbeddedArea(props: &AreaProps) -> Html {
    /*
         h("div", {
       style: {
         width: "100%",
         height: "100%",
         display: "flex",
         justifyContent: "center",
         alignItems: "center",
         backgroundColor: "lightblue",
         visibility: area.visible ? "visible" : "hidden",
         pointerEvents: interactive ? "auto" : "none",
       },
       onMouseEnter: () => {
         console.log("Mouse entered");
       },
       dangerouslySetInnerHTML: {
         __html: area.content,
       },
     }),
    */

    let visible = match &props.area.kind {
        AreaKind::Html(html) => html.visible,
        AreaKind::Canvas => panic!("Expected HTML area, not canvas area."),
    };

    let content = match &props.area.kind {
        AreaKind::Html(html) => html.content.as_str(),
        AreaKind::Canvas => panic!("Expected HTML area, not canvas area."),
    };

    let div_style = format!(
        "width: 100%; height: 100%; display: flex; justify-content: center; align-items: center; background-color: lightblue; visibility: {}; pointer-events: {}",
        if visible { "visible" } else { "hidden" },
        if props.interactive { "auto" } else { "none" }
    );

    // https://github.com/yewstack/yew/issues/3034
    // https://github.com/yewstack/yew/pull/3629

    html! {
        <@{"foreignObject"}
            x={props.area.x.to_string()}
            y={props.area.y.to_string()}
            width={props.area.width.to_string()}
            height={props.area.height.to_string()}
            style="overflow: hidden"
        >
            <div style={div_style} xmlns="http://www.w3.org/1999/xhtml">
                {Html::from_html_unchecked(content.to_string().into())}
            </div>
        </@>
    }
}

#[derive(Properties, PartialEq)]
struct MaskRectProps {
    area: Area,
}

#[function_component]
fn MaskRect(props: &MaskRectProps) -> Html {
    /*
        function MaskRect(area) {
      return h("rect", {
        key: area.id,
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
        // TODO: This roundness looks good for egui windows but should not be hardcoded.
        // In the future, a canvas area may not be a window.
        rx: area.kind === "canvas" ? 7 : 0,
        fill: area.kind === "canvas" ? "black" : "white",
      });
    }
         */

    let (rx, fill) = match &props.area.kind {
        AreaKind::Html(_) => (0, "white"),
        AreaKind::Canvas => (7, "black"),
    };

    html! {
        <rect
            x={props.area.x.to_string()}
            y={props.area.y.to_string()}
            width={props.area.width.to_string()}
            height={props.area.height.to_string()}
            rx={rx.to_string()}
            fill={fill}
        />
    }
}

#[function_component]
fn App() -> Html {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let window_width = use_state(|| window.inner_width().unwrap().as_f64().unwrap() as f32);
    let window_height = use_state(|| window.inner_height().unwrap().as_f64().unwrap() as f32);
    let mouse_x = use_state(|| 0.0);
    let mouse_y = use_state(|| 0.0);

    let areas = use_state(|| Vec::<Area>::new());

    let areas_clone = areas.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(areas) = RX.get().unwrap().lock().unwrap().next().await {
                areas_clone.set(areas);
            }
        });
    });

    let window_width_clone = window_width.clone();
    let window_height_clone = window_height.clone();
    let mouse_x_clone = mouse_x.clone();
    let mouse_y_clone = mouse_y.clone();
    use_effect_with((), move |_| {
        let handle_resize = Closure::wrap(Box::new(move || {
            let window = web_sys::window().unwrap();
            window_width_clone.set(window.inner_width().unwrap().as_f64().unwrap() as f32);
            window_height_clone.set(window.inner_height().unwrap().as_f64().unwrap() as f32);
        }) as Box<dyn Fn()>);
        window
            .add_event_listener_with_callback("resize", handle_resize.as_ref().unchecked_ref())
            .unwrap();

        // TODO: ?
        handle_resize.forget();

        let handle_mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            mouse_x_clone.set(event.client_x() as f32);
            mouse_y_clone.set(event.client_y() as f32);
        }) as Box<dyn Fn(_)>);

        document
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .add_event_listener_with_callback(
                "mousemove",
                handle_mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();

        window
            .add_event_listener_with_callback(
                "mousemove",
                handle_mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();

        handle_mouse_move.forget();

        // TODO: Cleanup.
    });

    /*
        function App() {
      const [windowWidth, setWindowWidth] = useState(window.innerWidth);
      const [windowHeight, setWindowHeight] = useState(window.innerHeight);
      const [mouseX, setMouseX] = useState(0);
      const [mouseY, setMouseY] = useState(0);
      const [areas, setAreas] = useState([]);
      window.setAreas = setAreas;

      useEffect(() => {
        const handleResize = () => {
          setWindowWidth(window.innerWidth);
          setWindowHeight(window.innerHeight);
        };

        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
      }, []);

      useEffect(() => {
        const handleMouseMove = (event) => {
          setMouseX(event.clientX);
          setMouseY(event.clientY);
        };

        // Because of the svg having no pointer events, only the canvas can read mousemove, document, window and body can't, idk why.
        // And this will not trigger if canvas is nor receving mousemove events.
        document
          .getElementById("the_canvas_id")
          .addEventListener("mousemove", handleMouseMove);

        // When the canvas does not receive mousemove, maybe other elements is capturing it and window should receive it.
        window.addEventListener("mousemove", handleMouseMove);

        return () => {
          document
            .getElementById("the_canvas_id")
            .removeEventListener("mousemove", handleMouseMove);

          window.removeEventListener("mousemove", handleMouseMove);
        };
      }, []);

      const hoveredArea = areas.findLast((area) => {
        return (
          area.x <= mouseX &&
          mouseX <= area.x + area.width &&
          area.y <= mouseY &&
          mouseY <= area.y + area.height
        );
      });
      console.log(mouseX, mouseY);
      console.log(hoveredArea);

      return h(
        "svg",
        {
          width: windowWidth,
          height: windowHeight,
          style: {
            position: "absolute",
            top: 0,
            left: 0,
            mask: "url(#mask)",
            zIndex: 1000,
            // Issue: This causes window mousemove events to be ignored.
            pointerEvents: "none",
          },
        },
        [
          h("defs", {}, [
            h(
              "mask",
              {
                id: "mask",
              },
              areas.map(MaskRect)
            ),
          ]),

          areas
            .filter((area) => area.kind === "html")
            .map((area) => Area({ area, interactive: area === hoveredArea })),
        ]
      );
    }
        */

    let hovered_area = areas.iter().rev().find(|area| {
        area.x <= *mouse_x
            && *mouse_x <= area.x + area.width
            && area.y <= *mouse_y
            && *mouse_y <= area.y + area.height
    });

    html! {
        <>

            <svg
                width={window_width.to_string()}
                height={window_height.to_string()}
                style="position: absolute; top: 0; left: 0; mask: url(#mask); z-index: 1000; pointer-events: none"
            >
                <defs>
                    <mask id="mask">
                        {for areas.iter().map(|area| html! { <MaskRect area={area.clone()} /> })}
                    </mask>
                </defs>

                {for areas.iter().filter(|area| matches!(area.kind, AreaKind::Html(_))).map(|area| html! { <EmbeddedArea area={area.clone()} interactive={Some(area) == hovered_area} /> })}
            </svg>
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
