use crate::CompositionContext;

pub(crate) trait CompositionStrategy: Send {
    fn compose(&mut self, cmp: &mut CompositionContext);
}
