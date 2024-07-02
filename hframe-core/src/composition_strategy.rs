use crate::{composition_context::CompositionContext, platform::Platform};

pub(crate) trait CompositionStrategy<P: Platform>: Send {
    fn name(&self) -> &'static str;
    fn compose(&mut self, cmp: &mut CompositionContext<P>);
}
