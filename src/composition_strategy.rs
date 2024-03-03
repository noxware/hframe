use crate::CompositionContext;

pub(crate) trait CompositionStrategy: Send {
    fn name(&self) -> &'static str;
    fn compose(&mut self, cmp: &mut CompositionContext);
}
