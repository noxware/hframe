use crate::{geo::Pos, platform::Platform};

pub(crate) struct TestPlatform {
    pub(crate) mouse_pos: Pos,
}

impl Platform for TestPlatform {
    fn mouse_pos(&self) -> Pos {
        self.mouse_pos
    }
}
