use std::{cell::RefCell, rc::Rc};

use crate::{geo::Pos, platform::Platform};

pub(crate) struct TestPlatformInner {
    pub(crate) mouse_pos: Pos,
}

pub(crate) struct TestPlatform(Rc<RefCell<TestPlatformInner>>);

impl Clone for TestPlatform {
    fn clone(&self) -> Self {
        TestPlatform(Rc::clone(&self.0))
    }
}

impl Platform for TestPlatform {
    fn mouse_pos(&self) -> Pos {
        self.0.borrow().mouse_pos
    }
}

impl TestPlatform {
    pub(crate) fn new() -> Self {
        TestPlatform(Rc::new(RefCell::new(TestPlatformInner {
            mouse_pos: Pos::new(0.0, 0.0),
        })))
    }

    pub(crate) fn set_mouse_pos(&self, mouse_pos: Pos) {
        self.0.borrow_mut().mouse_pos = mouse_pos;
    }
}
