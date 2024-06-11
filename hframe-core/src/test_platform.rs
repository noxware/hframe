use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    geo::Pos,
    platform::{Platform, PlatformEvent},
};

pub(crate) struct TestPlatformInner {
    pub(crate) events: Vec<PlatformEvent>,
}

pub(crate) struct TestPlatform(Rc<RefCell<TestPlatformInner>>);

impl Clone for TestPlatform {
    fn clone(&self) -> Self {
        TestPlatform(Rc::clone(&self.0))
    }
}

impl Platform for TestPlatform {
    fn clear_events(&mut self) {
        self.0.borrow_mut().events.clear();
    }

    fn events(&self) -> impl Iterator<Item = impl AsRef<PlatformEvent>> {
        self.0
            .borrow()
            .events
            .iter()
            .map(|e| e as &dyn AsRef<PlatformEvent>)
    }
}

impl TestPlatform {
    pub(crate) fn new() -> Self {
        TestPlatform(Rc::new(RefCell::new(TestPlatformInner {
            events: Vec::new(),
        })))
    }

    pub(crate) fn set_mouse_pos(&self, mouse_pos: Pos) {
        self.0.borrow_mut().mouse_pos = mouse_pos;
    }
}
