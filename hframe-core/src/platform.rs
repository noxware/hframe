use crate::geo::Pos;

pub(crate) trait Platform {
    fn mouse_pos(&self) -> Pos;
}
