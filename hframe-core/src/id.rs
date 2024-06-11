use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Id(u64);

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Id(id)
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        Id(hasher.finish())
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
