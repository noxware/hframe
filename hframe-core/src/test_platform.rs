use crate::{
    geo::Pos,
    platform::{Platform, PlatformEvent},
};

pub(crate) struct TestPlatform {
    events: Vec<PlatformEvent>,
}

impl Platform for TestPlatform {
    fn clear_events(&mut self) {
        self.events.clear();
    }

    fn events(&self) -> impl Iterator<Item = &PlatformEvent> {
        self.events.iter()
    }
}

impl TestPlatform {
    pub(crate) fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub(crate) fn move_pointer_to(&mut self, pos: Pos) {
        self.events.push(PlatformEvent::PointerMove(pos));
    }
}
