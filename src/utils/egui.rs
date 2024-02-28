use ::std::sync::{Arc, Mutex};
use std::ops::Deref;

macro_rules! eid {
    ($id:expr) => {
        egui::Id::new($id)
    };
}

pub(crate) use eid;

pub(crate) struct EguiCheap<T>(Arc<Mutex<T>>);

impl<T> EguiCheap<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }
}

impl<T> Deref for EguiCheap<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for EguiCheap<T> {
    fn clone(&self) -> Self {
        EguiCheap(self.0.clone())
    }
}

// Layzy attempt to detect dragging.
pub(crate) fn is_pointer_primary_down(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary))
}
