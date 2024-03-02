use web_sys::wasm_bindgen::JsCast;

use crate::{utils, CompositionContext, CompositionStrategy};

const MASK_TEMPLATE: &str = r#"
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

const HOLE_TEMPLATE: &str =
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />"#;

pub(crate) struct SvgDataMask;

impl SvgDataMask {
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        Self
    }
}

impl CompositionStrategy for SvgDataMask {
    fn compose(&mut self, cmp: &mut CompositionContext) {
        for area in cmp.get_composed_areas() {
            if area.html.is_none() {
                continue;
            }

            let area_rect = area.html.as_ref().unwrap().rect;
            let holes = cmp
                .get_composed_areas_on_top_of(area)
                .map(|hole| utils::geometry::rect_to_relative(hole.rect, area_rect));
            let area_rect = utils::geometry::rect_to_relative(area_rect, area_rect);

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
                .replace("{width}", &area_rect.width().to_string())
                .replace("{height}", &area_rect.height().to_string())
                .replace("{holes}", &holes);

            let encoded = format!("url(data:image/svg+xml,{})", urlencoding::encode(&svg));

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element = document
                .get_element_by_id(&area.html.as_ref().unwrap().id)
                .expect("Element to compose not found")
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            let style = element.style();
            style.set_property("mask", &encoded).unwrap();
            style.set_property("-webkit-mask", &encoded).unwrap();
        }
    }
}
