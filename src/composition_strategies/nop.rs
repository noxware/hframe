use crate::{CompositionContext, CompositionStrategy};

pub(crate) struct Nop;

impl CompositionStrategy for Nop {
    fn compose(_cmp: &mut CompositionContext) {}
}
