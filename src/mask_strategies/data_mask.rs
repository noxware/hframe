use crate::utils::rect_to_relative;
use crate::{
    html_window_state::HtmlWindowState,
    mask_strategy::{MaskStrategy, MaskStrategyMeta},
};
use std::any::Any;

pub const MASK_TEMPLATE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <defs>
    <mask id="mask" x="0" y="0" width="{width}" height="{height}">
      <rect x="0" y="0" width="{width}" height="{height}" fill="white" />
      {holes}      
    </mask>
  </defs>
  <rect x="0" y="0" width="{width}" height="{height}" fill="blue" mask="url(#mask)" />
</svg>
"#;

pub const HOLE_TEMPLATE: &str =
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />"#;

macro_rules! hframe_style {
    ($state:expr) => {
        format!(
            "top: {}px; left: {}px; width: {}px; height: {}px; {}; {}; mask: url({}); -webkit-mask: url({});",
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
            $state.mask.as_ref().unwrap().downcast_ref::<String>().unwrap(),
            $state.mask.as_ref().unwrap().downcast_ref::<String>().unwrap()
        )
    };
}

/// Strategy that produces SVG images encoded as data URIs that are set as HTML
/// elements masks.
///
/// A bit inneficient but should work well in everything except Firefox where a
/// more efficient approach can be used safely anyways.
pub struct DataMask;

impl Default for DataMask {
    fn default() -> Self {
        Self::new()
    }
}

impl DataMask {
    pub fn new() -> Self {
        Self
    }
}

impl MaskStrategy for DataMask {
    fn meta(&self) -> MaskStrategyMeta {
        MaskStrategyMeta {
            name: "data_mask".into(),
        }
    }

    fn setup(&self) {}

    fn cleanup(&self) {
        todo!("You can't change from this strategy")
    }

    fn compute_mask(
        &self,
        hframe: &HtmlWindowState,
        holes: &mut dyn Iterator<Item = egui::Rect>,
    ) -> Option<Box<dyn Any + Send>> {
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

        let svg = MASK_TEMPLATE
            .replace("{width}", &parent.width().to_string())
            .replace("{height}", &parent.height().to_string())
            .replace("{holes}", &holes);

        let encoded = format!("data:image/svg+xml,{}", urlencoding::encode(&svg));

        Some(Box::new(encoded))
    }

    fn mask(&self, state: &HtmlWindowState) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let element = document.get_element_by_id(&state.id).unwrap();

        let style = hframe_style!(state);
        element.set_attribute("class", "hframe").unwrap();
        element.set_attribute("style", &style).unwrap();
    }
}
