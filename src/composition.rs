use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use tap::prelude::*;

use crate::{
    area::{Area, AreaKind},
    companion, web,
};

pub(crate) struct Composition {
    pub(crate) areas: Vec<Area>,
}

impl Composition {
    pub(crate) fn new() -> Self {
        // TODO: Do this in other place.
        web::install();
        companion::install();
        Self { areas: vec![] }
    }

    pub(crate) fn sync(&mut self, egui_ctx: &egui::Context) {
        egui_ctx.memory(|mem| {
            self.areas = mem
                .layer_ids()
                .map(|layer_id| {
                    self.areas
                        .iter()
                        .filter(|area| area.layer_id == layer_id.id)
                        .collect::<Vec<_>>()
                        .tap_mut(|areas| {
                            areas.sort_by(|a, _b| {
                                if let AreaKind::Canvas = a.kind {
                                    std::cmp::Ordering::Less
                                } else {
                                    std::cmp::Ordering::Greater
                                }
                            })
                        })
                })
                .flatten()
                // TODO: Optimize.
                .cloned()
                .collect::<Vec<_>>();
        });

        companion::send_areas(self.areas.clone());
        web::send_areas(std::mem::take(&mut self.areas));
    }
}

pub(crate) struct CompositionHandle(Arc<Mutex<Composition>>);

impl Deref for CompositionHandle {
    type Target = Arc<Mutex<Composition>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for CompositionHandle {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub(crate) fn get_composition(ctx: &egui::Context) -> CompositionHandle {
    ctx.memory_mut(
        |mem| match mem.data.get_temp::<CompositionHandle>(egui::Id::NULL) {
            Some(cmp) => cmp,
            None => {
                let cmp = Composition::new();
                let cmp = CompositionHandle(Arc::new(Mutex::new(cmp)));
                mem.data.insert_temp(egui::Id::NULL, cmp.clone());
                cmp
            }
        },
    )
}

/// Syncs hframe internal stuff between the egui and web worlds. This function
/// **must be always called** at the end of the update loop unconditionally.
pub fn sync(ctx: &egui::Context) {
    let cmp = get_composition(ctx);
    cmp.lock().unwrap().sync(ctx);
}
