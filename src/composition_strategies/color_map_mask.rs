use crate::{ComposedAreaId, CompositionContext, CompositionStrategy};
use helpers::hash_into_rgba;
use std::collections::{HashMap, HashSet};
use web_sys::wasm_bindgen::JsCast;

mod helpers;
mod js_glue;

pub(crate) struct ColorMapMask {
    // Reading the previous mask directly from the element is not safe since hframe
    // can clean styles between cycles. That's why we must keep track of the previous
    // values manually.
    previous_masks: HashMap<ComposedAreaId, String>,
}

impl ColorMapMask {
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        js_glue::setup();
        Self {
            previous_masks: HashMap::new(),
        }
    }

    fn purge_previous_masks(&mut self, cmp: &CompositionContext) {
        let current_ids: HashSet<_> = cmp.get_composed_areas().iter().map(|a| a.id).collect();
        self.previous_masks.retain(|id, _| current_ids.contains(id));
    }
}

impl CompositionStrategy for ColorMapMask {
    fn name(&self) -> &'static str {
        "color_map_mask"
    }

    fn compose(&mut self, cmp: &mut CompositionContext) {
        // Clean tracking garbage to avoid memory leaks.
        self.purge_previous_masks(cmp);
        js_glue::clear();
        for area in cmp.get_composed_areas() {
            if area.html.is_none() {
                let rect = area.rect;
                js_glue::draw_rect(
                    rect.min.x.into(),
                    rect.min.y.into(),
                    rect.width().into(),
                    rect.height().into(),
                    "rgba(0, 0, 0, 1)",
                );

                continue;
            }

            let html = area.html.as_ref().unwrap();
            let rect = html.rect;
            let color = hash_into_rgba(&html.id);
            let color = format!(
                "rgba({}, {}, {}, {})",
                color.0,
                color.1,
                color.2,
                color.3 as f64 / 255.0
            );

            js_glue::draw_rect(
                rect.min.x.into(),
                rect.min.y.into(),
                rect.width().into(),
                rect.height().into(),
                &color,
            );
        }
    }
}
