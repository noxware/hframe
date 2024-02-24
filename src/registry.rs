use crate::aware::Awares;
use crate::mask_strategies;
use crate::mask_strategy::{MaskStrategy, MaskStrategyMeta};
use crate::utils::{eid, sync_hframe};
use crate::window::Window;
use crate::window_state::WindowState;
use std::collections::HashSet;

pub struct Registry {
    pub(crate) hframes: Vec<WindowState>,
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
    pub fn new() -> Self {
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

        Self {
            hframes: Vec::new(),
            hframe_awares: Awares::default(),
            hframes_since_last_sync: HashSet::new(),
            mask_strategy: Box::new(mask_strategies::Auto::new()),
        }
    }

    pub fn window<'reg>(&'reg mut self, id: &str, title: &str, content: &str) -> Window<'_, 'reg> {
        Window {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            open: None,
            registry: self,
        }
    }

    pub fn aware<R>(
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
                    let mut prev_rects = sorted_awares[0..index].iter().map(|(_, rect)| *rect);
                    hframe.mask = self.mask_strategy.compute_mask(&hframe, &mut prev_rects);
                }
            }
        });
    }

    pub fn sync(&mut self, ctx: &egui::Context) {
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

    pub fn set_mask_strategy<M: MaskStrategy + 'static>(&mut self, mask_strategy: M) {
        self.mask_strategy.cleanup();
        self.mask_strategy = Box::new(mask_strategy);
        self.mask_strategy.setup();
    }

    pub fn mask_strategy_meta(&self) -> MaskStrategyMeta {
        self.mask_strategy.meta()
    }
}
