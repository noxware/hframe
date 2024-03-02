use crate::{composition_strategies, utils, ComposedArea, CompositionStrategy};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

const GLOBAL_STYLES_ID: &str = "hframe-global-styles";

pub(crate) struct CompositionContext {
    // Hope this doesn't cause a cycle reference. I don't see right know a way
    // to make this weak.
    pub(crate) egui_ctx: egui::Context,
    composed_areas: Vec<ComposedArea>,
    /// `dyn` to support setting a strategy with a runtime criteria.
    composition_strategy: Option<Box<dyn CompositionStrategy>>,
}

impl CompositionContext {
    pub(crate) fn new(egui_ctx: &egui::Context) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let style = document.get_element_by_id(GLOBAL_STYLES_ID);

        if style.is_none() {
            document
                .head()
                .expect("No head element found in the document")
                .insert_adjacent_html(
                    "beforeend",
                    &format!(
                        "<style id=\"{}\">{}</style>",
                        GLOBAL_STYLES_ID,
                        include_str!("hframe.css")
                    ),
                )
                .unwrap();
        }

        Self {
            egui_ctx: egui_ctx.clone(),
            composed_areas: Vec::new(),
            composition_strategy: Some(Box::new(composition_strategies::SvgDataMask::new())),
        }
    }

    pub(crate) fn put_composed_area(&mut self, area: ComposedArea) {
        let (new, prev) = utils::vec::insert_or_replace(&mut self.composed_areas, area, |a| a.id);

        if let Some(new_html) = &new.html {
            let did_content_change = prev
                .as_ref()
                .map(|prev| {
                    prev.html
                        .as_ref()
                        .expect("Non HTML area turned into HTML area")
                        .content
                        != new_html.content
                })
                .unwrap_or(true);

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let element = document.get_element_by_id(&new_html.id).unwrap_or_else(|| {
                let body = document.body().unwrap();
                let element = document.create_element("div").unwrap();
                body.append_child(&element).unwrap();
                element
            });

            if did_content_change {
                element.set_outer_html(&new_html.to_outer_html());
            } else {
                element
                    .set_attribute("style", &new_html.to_styles())
                    .unwrap();
            }
        }
    }

    pub(crate) fn compose(&mut self) {
        let layer_ids: Vec<_> = self.egui_ctx.memory(|mem| mem.layer_ids().collect());

        let mut composed_areas = std::mem::take(&mut self.composed_areas);
        self.composed_areas = layer_ids
            .iter()
            .filter_map(|layer_id| {
                composed_areas
                    .iter()
                    .position(|area| area.id == layer_id.id)
                    .map(|pos| composed_areas.swap_remove(pos))
            })
            .collect();

        if let Some(mut strategy) = self.composition_strategy.take() {
            strategy.compose(self);
            self.composition_strategy = Some(strategy);
        }
    }

    pub(crate) fn get_composed_areas(&self) -> &[ComposedArea] {
        &self.composed_areas
    }

    /// Only safe to call in `compose` phase where the areas are known and
    /// sorted.
    pub(crate) fn get_composed_areas_on_top_of(&self, of: &ComposedArea) -> &[ComposedArea] {
        let index = self
            .composed_areas
            .iter()
            .position(|area| area.id == of.id)
            .expect("The are is not known in this composition context");

        // ???
        &self.composed_areas[index + 1..]
    }
}

impl Drop for CompositionContext {
    fn drop(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let style = document.get_element_by_id(GLOBAL_STYLES_ID);
        if let Some(style) = style {
            style.remove();
        }
    }
}

#[derive(Clone)]
pub(crate) struct WrappedCompositionContext(pub(crate) Arc<Mutex<CompositionContext>>);

impl Deref for WrappedCompositionContext {
    type Target = Arc<Mutex<CompositionContext>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) fn get_composition_context(ctx: &egui::Context) -> WrappedCompositionContext {
    ctx.memory_mut(|mem| {
        match mem
            .data
            .get_temp::<WrappedCompositionContext>(egui::Id::NULL)
        {
            Some(cmp) => cmp,
            None => {
                let cmp = CompositionContext::new(ctx);
                let cmp = WrappedCompositionContext(Arc::new(Mutex::new(cmp)));
                mem.data.insert_temp(egui::Id::NULL, cmp.clone());
                cmp
            }
        }
    })
}

pub fn sync(ctx: &egui::Context) {
    let cmp = get_composition_context(ctx);
    let mut cmp = cmp.lock().unwrap();
    cmp.compose();
}
