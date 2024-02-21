use crate::utils::rect_to_relative;
use crate::{mask_strategy::MaskStrategy, window_state::WindowState};
use std::any::Any;

pub const MASK_TEMPLATE: &str = r#"
<svg id="{id}-svg" class="hframe-mask-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <defs>
    <mask id="{id}-mask" x="0" y="0" width="{width}" height="{height}">
      <rect x="0" y="0" width="{width}" height="{height}" fill="white" />
      {holes}      
    </mask>
  </defs>
</svg>
"#;

pub const HOLE_TEMPLATE: &str =
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />"#;

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

pub struct DocumentMask;

impl DocumentMask {
    pub fn new() -> Self {
        Self
    }
}

impl MaskStrategy for DocumentMask {
    fn setup(&self) {}

    fn cleanup(&self) {
        todo!("You can't change from this strategy")
    }

    fn compute_mask(
        &self,
        hframe: &WindowState,
        holes: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any>> {
        let parent = hframe.rect;
        let holes = holes.map(|hole| rect_to_relative(hole, parent));
        let parent = rect_to_relative(parent, parent);

        let holes = holes
            .map(|hole| {
                HOLE_TEMPLATE
                    .replace("{x}", &hole.min.x.to_string())
                    .replace("{y}", &hole.min.y.to_string())
                    .replace("{width}", &hole.width().to_string())
                    .replace("{height}", &hole.height().to_string())
            })
            .collect::<String>();

        let svg_mask = MASK_TEMPLATE
            .replace("{id}", &hframe.id)
            .replace("{width}", &parent.width().to_string())
            .replace("{height}", &parent.height().to_string())
            .replace("{holes}", &holes);

        Some(Box::new(svg_mask))
    }

    fn mask(&self, state: &WindowState) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.get_element_by_id(&state.id).unwrap();

        let svg = document
            .get_element_by_id(&format!("{}-svg", state.id))
            .unwrap_or_else(|| {
                let svg = document.create_element("svg").unwrap();
                body.append_child(&svg).unwrap();
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
