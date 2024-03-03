use crate::{utils, CompositionContext, CompositionStrategy};
use std::collections::{HashMap, HashSet};
use web_sys::wasm_bindgen::JsCast;

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

pub(crate) struct SvgDataMask {
    // Reading the previous mask directly from the element is not safe since hframe
    // can clean styles between cycles. That's why we must keep track of the previous
    // values manually.
    previous_masks: HashMap<egui::Id, String>,
}

impl SvgDataMask {
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        Self {
            previous_masks: HashMap::new(),
        }
    }

    fn purge_previous_masks(&mut self, cmp: &CompositionContext) {
        let current_ids: HashSet<_> = cmp.get_composed_areas().iter().map(|a| a.id).collect();
        self.previous_masks.retain(|id, _| current_ids.contains(id));
    }
}

impl CompositionStrategy for SvgDataMask {
    fn name(&self) -> &'static str {
        "svg_data_mask"
    }

    fn compose(&mut self, cmp: &mut CompositionContext) {
        // Clean tracking garbage to avoid memory leaks.
        self.purge_previous_masks(cmp);
        for area in cmp.get_composed_areas() {
            if area.html.is_none() {
                continue;
            }

            let area_html = area.html.as_ref().unwrap();

            let area_rect = area_html.rect;

            let holes: Vec<_> = cmp.get_composed_areas_on_top_of(area).collect();

            let hole_rects: Vec<_> = holes
                .iter()
                .map(|hole| utils::geometry::rect_to_relative(hole.rect, area_rect))
                .collect();
            let area_rect = utils::geometry::rect_to_relative(area_rect, area_rect);

            let mask = compute_mask(area_rect, &hole_rects);

            let document = web_sys::window().unwrap().document().unwrap();

            let element = document
                .get_element_by_id(&area.html.as_ref().unwrap().id)
                .expect("Element to compose not found")
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            let prev_mask = self.previous_masks.get(&area.id);

            let area_being_dragged = cmp.get_composed_area_being_dragged();
            let is_hole_being_dragged =
                area_being_dragged.map_or(false, |a| holes.iter().any(|h| h.id == a.id));

            if is_hole_being_dragged
                && !utils::browser_detection::is_blink()
                && prev_mask != Some(&mask)
            {
                area_html
                    .get_html_element()
                    .style()
                    .set_property("visibility", "hidden")
                    .unwrap();
            } else {
                let style = element.style();
                style.set_property("mask", &mask).unwrap();
                style.set_property("-webkit-mask", &mask).unwrap();
                self.previous_masks.insert(area.id, mask);
            }
        }
    }
}

fn compute_mask(area_rect: egui::Rect, hole_rects: &[egui::Rect]) -> String {
    let holes = hole_rects
        .iter()
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

    format!("url(data:image/svg+xml,{})", urlencoding::encode(&svg))
}
