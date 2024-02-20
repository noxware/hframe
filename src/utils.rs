use crate::templates;
use crate::window_state::WindowState;

macro_rules! eid {
    ($id:expr) => {
        egui::Id::new($id)
    };
}

pub(crate) use eid;

macro_rules! hframe_style {
    ($state:expr) => {
        format!(
            "top: {}px; left: {}px; width: {}px; height: {}px; {}; {}; mask: url(#{});",
            $state.rect.min.y,
            $state.rect.min.x,
            $state.rect.width(),
            $state.rect.height(),
            if $state.interactable {
                ""
            } else {
                "pointer-events: none;"
            },
            if $state.visible {
                ""
            } else {
                "visibility: hidden;"
            },
            format!("{}-mask", $state.id)
        )
    };
}

fn rect_to_relative(rect: egui::Rect, parent: egui::Rect) -> egui::Rect {
    let min = rect.min - parent.min;
    let max = rect.max - parent.min;
    egui::Rect::from_min_max(min.to_pos2(), max.to_pos2())
}

pub(crate) fn build_mask_svg<H: Iterator<Item = egui::Rect>>(
    hframe: &WindowState,
    holes: H,
) -> String {
    let parent = hframe.rect;
    let holes = holes.map(|hole| rect_to_relative(hole, parent));
    let parent = rect_to_relative(parent, parent);

    let holes = holes
        .map(|hole| {
            templates::HOLE_TEMPLATE
                .replace("{x}", &hole.min.x.to_string())
                .replace("{y}", &hole.min.y.to_string())
                .replace("{width}", &hole.width().to_string())
                .replace("{height}", &hole.height().to_string())
        })
        .collect::<String>();

    templates::MASK_TEMPLATE
        .replace("{id}", &hframe.id)
        .replace("{width}", &parent.width().to_string())
        .replace("{height}", &parent.height().to_string())
        .replace("{holes}", &holes)
}

pub(crate) fn sync_hframe(state: &WindowState) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let element = document.get_element_by_id(&state.id).unwrap_or_else(|| {
        let body = document.body().unwrap();
        let hframe = document.create_element("div").unwrap();
        hframe.set_attribute("id", &state.id).unwrap();
        hframe.set_inner_html(&state.content);
        body.append_child(&hframe).unwrap();
        hframe
    });

    if state.content_changed {
        element.set_inner_html(&state.content);
    }

    let svg = document
        .get_element_by_id(&format!("{}-svg", state.id))
        .unwrap_or_else(|| {
            let svg = document.create_element("svg").unwrap();
            body.append_child(&svg).unwrap();
            svg
        });

    svg.set_outer_html(&state.mask);

    let style = hframe_style!(state);
    element.set_attribute("class", "hframe").unwrap();
    element.set_attribute("style", &style).unwrap();
}
