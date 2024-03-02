use crate::CompositionContext;

pub(crate) trait CompositionStrategy {
    fn compose(cmp: &mut CompositionContext);
}
