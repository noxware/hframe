#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Pos {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Pos {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Size {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Size {
    pub(crate) fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Rect {
    pub(crate) pos: Pos,
    pub(crate) size: Size,
}

impl From<(Pos, Size)> for Rect {
    fn from((pos, size): (Pos, Size)) -> Self {
        Rect { pos, size }
    }
}

impl From<(f64, f64, f64, f64)> for Rect {
    fn from((x, y, width, height): (f64, f64, f64, f64)) -> Self {
        Rect {
            pos: Pos::new(x, y),
            size: Size::new(width, height),
        }
    }
}
