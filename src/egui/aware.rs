/// Allows you to implement `aware` for egui entities so hframe can know about
/// their existence when applying compositions.
pub trait Aware {
    /// Let hframe know about the existence of this entity.
    ///
    /// You must call this in anything from the egui world that can overlap
    /// with HTML content (like normal egui windows).
    fn aware(self) -> Self;
}

impl<R> Aware for Option<egui::InnerResponse<R>> {
    fn aware(self) -> Self {
        let inner_response = self?;
        Some(inner_response)
    }
}
