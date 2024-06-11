use crate::geo::Pos;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub(crate) enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

#[derive(Debug, Clone)]
pub(crate) enum PlatformEvent {
    PointerMove(Pos),
    PointerDown(PointerButton),
    PointerUp(PointerButton),
}

pub(crate) trait Platform {
    fn events(&self) -> impl Iterator<Item = impl AsRef<PlatformEvent>>;
    fn clear_events(&mut self);
}
