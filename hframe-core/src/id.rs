use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Mutex, OnceLock},
};

static DEBUG_ID_REGISTRY: OnceLock<Mutex<HashMap<Id, String>>> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Id(u64);

impl Id {
    /// A semantic id representing the root of a tree-like resource.
    pub(crate) fn root() -> Self {
        Id(42)
    }

    fn debug_name(&self) -> Option<String> {
        DEBUG_ID_REGISTRY.get()?.lock().ok()?.get(self).cloned()
    }

    fn set_debug_name(&self, name: String) {
        DEBUG_ID_REGISTRY
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .insert(*self, name);
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

        // TODO: Allow debug names outside of tests.
        if cfg!(test) {
            result.set_debug_name(id.to_string());
        }

        result
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(name) = self.debug_name() {
            write!(f, "{}({})", self.0, name)
        } else {
            write!(f, "{}", self.0)
        }
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
