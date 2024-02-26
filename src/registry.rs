use crate::aware::Awares;
use crate::mask_strategies;
use crate::utils::{eid, sync_hframe, EguiCheap};
use crate::{HtmlWindowState, MaskStrategy, MaskStrategyMeta};
use std::collections::HashSet;

pub(crate) struct Registry {
    pub(crate) hframes: Vec<HtmlWindowState>,
    pub(crate) hframe_awares: Awares,
    pub(crate) hframes_since_last_sync: HashSet<String>,
    mask_strategy: Box<dyn MaskStrategy>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    fn new() -> Self {
        let style = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("#hframe-shared-styles")
            .unwrap();

        if style.is_none() {
            let head = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .head()
                .unwrap();

            head.insert_adjacent_html(
                "beforeend",
                &format!(
                    "<style id=\"hframe-shared-styles\">{}</style>",
                    include_str!("hframe.css")
                ),
            )
            .unwrap();
        }

        let mut res = Self {
            hframes: Vec::new(),
            hframe_awares: Awares::default(),
            hframes_since_last_sync: HashSet::new(),
            mask_strategy: Box::new(mask_strategies::Nop::new()),
        };

        res.set_mask_strategy(mask_strategies::Auto::new());
        res
    }

    fn aware<R>(
        &mut self,
        inner_response: Option<egui::InnerResponse<R>>,
    ) -> Option<egui::InnerResponse<R>> {
        self.hframe_awares.insert(inner_response)
    }

    fn clip(&mut self, ctx: &egui::Context) {
        ctx.memory(|mem| {
            let sorted_awares = mem
                .layer_ids()
                .filter_map(|layer_id| {
                    self.hframe_awares
                        .0
                        .get(&layer_id.id)
                        .map(|aware| (layer_id.id, aware.rect))
                })
                .collect::<Vec<_>>();

            let sorted_awares = sorted_awares.iter().rev().collect::<Vec<_>>();

            for (index, (id, _rect)) in sorted_awares.iter().enumerate() {
                if let Some(hframe) = self
                    .hframes
                    .iter_mut()
                    .find(|hframe| eid!(&hframe.id) == *id)
                {
                    let prev_rects = sorted_awares[0..index].iter().map(|(_, rect)| *rect);
                    let mut overlaping_rects = prev_rects.filter(|r| r.intersects(hframe.rect));
                    hframe.mask = self
                        .mask_strategy
                        .compute_mask(hframe, &mut overlaping_rects);
                }
            }
        });
    }

    fn sync(&mut self, ctx: &egui::Context) {
        self.clip(ctx);
        for state in &self.hframes {
            sync_hframe(state, &*self.mask_strategy);
        }
        self.clean();
    }

    fn clean(&mut self) {
        self.hframes.retain(|state| {
            if !self.hframes_since_last_sync.contains(&state.id) {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let element = document.get_element_by_id(&state.id).unwrap();
                element.remove();
                self.hframe_awares.0.remove(&eid!(&state.id));
                false
            } else {
                true
            }
        });

        self.hframes_since_last_sync.clear();
    }

    fn set_mask_strategy<M: MaskStrategy + 'static>(&mut self, mask_strategy: M) {
        self.mask_strategy.cleanup();
        for state in &mut self.hframes {
            state.mask = None;
        }
        self.mask_strategy = Box::new(mask_strategy);
        self.mask_strategy.setup();
    }

    fn mask_strategy_meta(&self) -> MaskStrategyMeta {
        self.mask_strategy.meta()
    }
}

fn create_cheap_registry() -> EguiCheap<Registry> {
    EguiCheap::new(Registry::new())
}

pub(crate) fn get_or_insert_registry(ctx: &egui::Context) -> EguiCheap<Registry> {
    ctx.memory_mut(
        |mem| match mem.data.get_temp::<EguiCheap<Registry>>(egui::Id::NULL) {
            Some(registry) => registry,
            None => {
                let registry = create_cheap_registry();
                mem.data.insert_temp(egui::Id::NULL, registry.clone());
                registry
            }
        },
    )
}

fn aware<R>(inner_response: Option<egui::InnerResponse<R>>) -> Option<egui::InnerResponse<R>> {
    let inner_response = inner_response?;
    let ctx = &inner_response.response.ctx;

    let reg = get_or_insert_registry(ctx);
    let mut reg = reg.lock().unwrap();
    reg.aware(Some(inner_response))
}

/// Syncs hframe internal stuff between the egui and web worlds. This function
/// **must be always called** at the end of the update loop unconditionally.
pub fn sync(ctx: &egui::Context) {
    let reg = get_or_insert_registry(ctx);
    let mut reg = reg.lock().unwrap();
    reg.sync(ctx);
}

/// Get the meta information of the masking strategy currently in use.
///
/// This is useful if you need to know which strategy is actually being used
/// when using the `Auto` strategy.
pub fn mask_strategy_meta(ctx: &egui::Context) -> MaskStrategyMeta {
    let reg = get_or_insert_registry(ctx);
    let reg = reg.lock().unwrap();
    reg.mask_strategy_meta()
}

/// Allows you to set a specific mask strategy at runtime.
pub fn set_mask_strategy<M: MaskStrategy + 'static>(ctx: &egui::Context, mask_strategy: M) {
    let reg = get_or_insert_registry(ctx);
    let mut reg = reg.lock().unwrap();
    reg.set_mask_strategy(mask_strategy);
}

/// Allows you to implement `aware` for egui entities so hframe can know about
/// their existence when computing masks and handling events.
pub trait Aware {
    /// Let hframe know about the existence of this entity.
    ///
    /// You must call this in anything from the egui world that can overlap
    /// with HTML content (like normal egui windows).
    fn aware(self) -> Self;
}

impl<R> Aware for Option<egui::InnerResponse<R>> {
    fn aware(self) -> Self {
        aware(self)
    }
}
