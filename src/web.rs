use crate::area::Area;

mod canvas;
mod compositor;
mod dom;
mod js;

// TODO: This intermidiate function is not needed.
pub(crate) fn install() {
    compositor::install();
}

// TODO: This intermidiate function is not needed.
pub(crate) fn send_areas(areas: Vec<Area>) {
    compositor::sync(areas);
}
