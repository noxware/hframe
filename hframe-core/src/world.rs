use crate::{
    composed_area::{ComposedArea, ComposedAreaKind},
    geo::Rect,
    id::Id,
    tree::Node,
};

pub(crate) struct World(Node<ComposedArea>);

impl World {
    pub(crate) fn new(viewport: Rect) -> Self {
        World(Node::new(ComposedArea {
            id: Id::root(),
            size: viewport.size,
            abs_pos: viewport.pos,
            kind: ComposedAreaKind::Canvas,
        }))
    }

    /// Getter for the root node of this world.
    pub(crate) fn root(&self) -> Node<ComposedArea> {
        self.0.clone()
    }

    /// Fetch a node from the tree by it's Id. Returns None if no node with the given Id is found.
    pub(crate) fn get(&self, id: Id) -> Option<Node<ComposedArea>> {
        self.0.find(|node| node.read(|data| data.value.id == id))
    }

    /// Add a new node to the tree under a specific Id.
    /// If the parent Id is not found, this function will panic.
    pub(crate) fn add(&mut self, parent_id: Id, area: ComposedArea) {
        let parent = self
            .0
            .find(|node| node.read(|data| data.value.id == parent_id))
            .expect("parent not found");

        parent.nest(Node::new(area));
    }
}
