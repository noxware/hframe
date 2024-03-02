macro_rules! eid {
    ($id:expr) => {
        egui::Id::new($id)
    };
}

pub(crate) use eid;
