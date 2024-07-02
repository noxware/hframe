use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Id(u64);

impl Id {
    /// A semantic id representing the root of a tree-like resource.
    pub(crate) fn root() -> Self {
        Id(42)
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Id(id)
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let result = Id(hasher.finish());
        eprintln!("Id::from({:?}) = {:?}", id, result);
        result
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
