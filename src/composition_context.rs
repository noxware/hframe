use crate::{composition_strategies, utils, ComposedArea, CompositionStrategy};
use std::{
    collections::HashSet,
    ops::Deref,
    sync::{Arc, Mutex},
};

pub(crate) struct CompositionContext {
    // Hope this doesn't cause a cycle reference. I don't see right know a way
    // to make this weak.
    pub(crate) egui_ctx: egui::Context,
    composed_areas: Vec<ComposedArea>,
    composed_areas_since_last_sync: HashSet<egui::Id>,
    /// `dyn` to support setting a strategy with a runtime criteria.
    composition_strategy: Option<Box<dyn CompositionStrategy>>,
}

impl CompositionContext {
    pub(crate) fn new(egui_ctx: &egui::Context) -> Self {
        let composition_strategy: Box<dyn CompositionStrategy> =
            Box::new(composition_strategies::SvgDataMask::new());

        Self {
            egui_ctx: egui_ctx.clone(),
            composed_areas: Vec::new(),
            composed_areas_since_last_sync: HashSet::new(),
            composition_strategy: Some(composition_strategy),
        }
    }

    pub(crate) fn put_composed_area(&mut self, area: ComposedArea) {
        let (new, prev) = utils::vec::insert_or_replace(&mut self.composed_areas, area, |a| a.id);
        self.composed_areas_since_last_sync.insert(new.id);
    }

    fn purge_composed_areas(&mut self) {
        self.composed_areas.retain(|a| {
            if !self.composed_areas_since_last_sync.contains(&a.id) {
                if let Some(html) = &a.html {
                    // TODO: Remove the HTML element from the DOM.
                }
                false
            } else {
                true
            }
        });

        self.composed_areas_since_last_sync.clear();
    }

    fn sort_composed_areas(&mut self) {
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
    }

    fn compose(&mut self) {
        if let Some(mut strategy) = self.composition_strategy.take() {
            strategy.compose(self);
            self.composition_strategy = Some(strategy);
        }
    }

    pub(crate) fn sync(&mut self) {
        self.purge_composed_areas();
        self.sort_composed_areas();
        self.compose();
    }

    pub(crate) fn get_composed_areas(&self) -> &[ComposedArea] {
        &self.composed_areas
    }

    /// Only safe to call in `compose` phase where the areas are known and
    /// sorted.
    pub(crate) fn get_composed_areas_on_top_of<'cmp>(
        &'cmp self,
        of: &'cmp ComposedArea,
    ) -> impl Iterator<Item = &'cmp ComposedArea> + 'cmp {
        let index = self
            .composed_areas
            .iter()
            .position(|area| area.id == of.id)
            .expect("The are is not known in this composition context");

        // This will also consider the non-HTML area part which may not be
        // relevant for compositions, but makes this function generic.
        self.composed_areas[index + 1..]
            .iter()
            .filter(|area| area.rect.intersects(of.rect))
    }

    pub(crate) fn get_composed_area_being_dragged(&self) -> Option<&ComposedArea> {
        let egui_ctx = &self.egui_ctx;
        // Lazy detection of dragging.
        let dragging = egui_ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary));

        if !dragging {
            return None;
        }

        let top_layer_id = egui_ctx.top_layer_id()?.id;
        self.composed_areas
            .iter()
            .find(|area| area.id == top_layer_id)
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

/// Syncs hframe internal stuff between the egui and web worlds. This function
/// **must be always called** at the end of the update loop unconditionally.
pub fn sync(ctx: &egui::Context) {
    let cmp = get_composition_context(ctx);
    let mut cmp = cmp.lock().unwrap();
    cmp.sync();
}
