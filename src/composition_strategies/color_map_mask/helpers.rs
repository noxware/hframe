#[derive(Clone, Copy)]
pub(super) struct Color {
    r: u8,
    g: u8,
    b: u8,
    #[allow(dead_code)]
    a: u8,
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

use std::hash::{DefaultHasher, Hash, Hasher};

fn hash_into_rgb(input: &str) -> (u8, u8, u8) {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    let r = (hash & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = ((hash >> 16) & 0xFF) as u8;

    (r, g, b)
}

pub fn hash_into_rgba(input: &str) -> (u8, u8, u8, u8) {
    if input.is_empty() {
        return (0, 0, 0, 0);
    }

    let (r, g, b) = hash_into_rgb(input);
    (r, g, b, 255)
}
