use web_sys::wasm_bindgen::JsCast;

const OVERLAYS_CONTAINER: &str = r#"
<div id="overlays-container" style="position: absolute; top: 0; left: 0; width: 0; height: 0; overflow: visible; pointer-events: none;"></div>
"#;

const OVERLAY_TEMPLATE: &str = r#"
<div id="{id}" style="{style}">
<canvas width="{width}" height="{height}"></canvas>
</div>
"#;

const OVERLAY_STYLE_TEMPLATE: &str =
    "top: {top}px; left: {left}px; width: {width}px; height: {height}px; z-index: {z_index}; background-color: green; position: absolute;";

pub(crate) fn setup() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    container.set_outer_html(OVERLAYS_CONTAINER);
}

fn try_get_container() -> Option<web_sys::Element> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.get_element_by_id("overlays-container")
}

fn get_container() -> web_sys::Element {
    try_get_container().unwrap()
}

fn overlay_id(id: egui::Id) -> String {
    // TODO: This should use `id.value()` in egui 0.26.x.
    // Consider serialize and deserialize if not possible, but
    // `short_debug_format` is probably not a good idea.
    format!("{}-overlay", id.short_debug_format())
}

fn try_get_overlay(id: egui::Id) -> Option<web_sys::Element> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.get_element_by_id(&overlay_id(id))
}

fn get_overlay(id: egui::Id) -> web_sys::Element {
    try_get_overlay(id).unwrap()
}

fn get_overlay_canvas(id: egui::Id) -> web_sys::HtmlCanvasElement {
    let overlay = get_overlay(id);
    let canvas = overlay.query_selector("canvas").unwrap().unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

fn get_egui_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    // TODO: Not guaranteed id.
    let canvas = document.get_element_by_id("the_canvas_id").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

fn capture_rect_into_overlay(id: egui::Id, rect: egui::Rect) {
    let egui_canvas = get_egui_canvas();
    let overlay_canvas = get_overlay_canvas(id);
    let overlay_context = overlay_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let sx = rect.min.x as f64;
    let sy = rect.min.y as f64;
    let sw = rect.width() as f64;
    let sh = rect.height() as f64;

    let dw = sw;
    let dh = sh;

    overlay_context
        .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &egui_canvas,
            sx,
            sy,
            sw,
            sh,
            0.,
            0.,
            dw,
            dh,
        )
        .unwrap();
}

pub(crate) fn create_or_update_overlay(
    egui_id: egui::Id,
    rect: egui::Rect,
    z_index: i64,
) -> web_sys::Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let container = get_container();

    let overlay = try_get_overlay(egui_id).unwrap_or_else(|| {
        let overlay = document.create_element("div").unwrap();
        container.append_child(&overlay).unwrap();
        overlay
    });

    let id = overlay_id(egui_id);

    let z_index = z_index + 1000;

    let style = OVERLAY_STYLE_TEMPLATE
        .replace("{top}", &rect.min.y.to_string())
        .replace("{left}", &rect.min.x.to_string())
        .replace("{width}", &rect.width().to_string())
        .replace("{height}", &rect.height().to_string())
        .replace("{z_index}", &z_index.to_string());

    let content = OVERLAY_TEMPLATE
        .replace("{id}", &id)
        .replace("{style}", &style)
        .replace("{width}", &rect.width().to_string())
        .replace("{height}", &rect.height().to_string());

    overlay.set_outer_html(&content);

    capture_rect_into_overlay(egui_id, rect);

    overlay
}

pub(crate) fn destroy_overlay(id: egui::Id) {
    let overlay = get_overlay(id);
    overlay.remove();
}

/*
pub struct CanvasOverlay;

impl CanvasOverlay {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl MaskStrategy for CanvasOverlay {
    fn meta(&self) -> MaskStrategyMeta {
        MaskStrategyMeta {
            name: "canvas_overlay".into(),
        }
    }

    fn setup(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let container = document.create_element("div").unwrap();
        body.append_child(&container).unwrap();

        container.set_outer_html(OVERLAYS_CONTAINER);
    }

    fn cleanup(&self) {
        self.get_container().remove();
    }

    fn compute_mask(
        &self,
        hframe: &HtmlWindowState,
        holes: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
        let holes = holes.peekable();

        if (holes.peek().is_none()) {}
    }

    fn mask(&self, state: &HtmlWindowState) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let element = document.get_element_by_id(&state.id).unwrap();

        let svg = document
            .get_element_by_id(&format!("{}-svg", state.id))
            .unwrap_or_else(|| {
                let svg = document.create_element("svg").unwrap();
                let container = document.get_element_by_id("masks-container").unwrap();
                container.append_child(&svg).unwrap();
                svg
            });

        svg.set_outer_html(
            state
                .mask
                .as_ref()
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
        );

        let style = hframe_style!(state);
        element.set_attribute("class", "hframe").unwrap();
        element.set_attribute("style", &style).unwrap();
    }
}
*/
