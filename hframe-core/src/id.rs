use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Id(Arc<str>);

impl Id {
    /// A semantic id representing the root of a tree-like resource.
    pub(crate) fn root() -> Self {
        static ROOT: OnceLock<Id> = OnceLock::new();
        ROOT.get_or_init(|| Id::from("__root__")).clone()
    }

    /// An id with no useful information. It acts as a "zero" id.
    ///
    /// Useful to simplify some algorithms.
    pub(crate) fn empty() -> Self {
        static EMPTY: OnceLock<Id> = OnceLock::new();
        EMPTY.get_or_init(|| Id::from("__empty__")).clone()
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Id(Arc::from(id))
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Id(Arc::from(id))
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Id(Arc::from(format!("__uint__{}", id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let id = Id::from("hello");
        assert_eq!(id, Id::from("hello"));
        assert_ne!(id, Id::from("world"));

        let id = Id::from(42);
        assert_eq!(id, Id::from(42));
        assert_ne!(id, Id::from(43));
    }
}
