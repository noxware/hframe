use crate::platform::Platform;

pub struct WebPlatform;

impl Platform for WebPlatform {
    fn mouse_pos(&self) -> crate::geo::Pos {
        crate::geo::Pos::new(0.0, 0.0)
    }
}
