use crate::{CompositionContext, CompositionStrategy};

pub(crate) struct Nop;

impl Nop {
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        Self
    }
}

impl CompositionStrategy for Nop {
    fn compose(&mut self, _cmp: &mut CompositionContext) {}
}
