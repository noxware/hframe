use crate::{
    composed_area::ComposedArea,
    geo::Pos,
    id::Id,
    platform::{Platform, PlatformEvent},
    tree::{Node, Walk},
    world::World,
};

/// Hold the state of the `hframe` world, the platform abstraction for reading events,
/// provides queries for different info, etc.
struct CompositionContext<P: Platform> {
    world: World,
    platform: P,
    pointer_pos: Pos,
}

impl<P: Platform> CompositionContext<P> {
    /// Returns the area that is currently under the pointer.
    fn get_hovered_area(&self) -> Option<Node<ComposedArea>> {
        let mouse_pos = self.pointer_pos;
        self.world.root().find_last(|node| {
            node.read(|data| {
                let area = &data.value;
                area.abs_pos.x <= mouse_pos.x
                    && area.abs_pos.x + area.size.width >= mouse_pos.x
                    && area.abs_pos.y <= mouse_pos.y
                    && area.abs_pos.y + area.size.height >= mouse_pos.y
            })
        })
    }

    /// Gets the area that is currently marked as "under attention".
    ///
    /// If `sync` was executed, this should be the same as the hovered area.
    /// If not, it may be different as the state is outdated.
    fn get_under_attention_area(&self) -> Option<Node<ComposedArea>> {
        self.world
            .root()
            .find(|node| node.read(|data| data.value.state.is_under_attention))
    }

    /// Processes pending events, and clear them.
    fn process_events(&mut self) {
        for event in self.platform.events() {
            match event {
                PlatformEvent::PointerMove(pos) => {
                    self.pointer_pos = *pos;
                }
                _ => {}
            }
        }

        self.platform.clear_events();
    }

    /// Using the current state from processing events, updates the data of the nodes.
    fn update_nodes(&mut self) {
        if let Some(prev) = self.get_under_attention_area() {
            prev.read_mut(|data| data.value.state.is_under_attention = false);
        }

        if let Some(hovered) = self.get_hovered_area() {
            hovered.read_mut(|data| data.value.state.is_under_attention = true);
        }
    }

    /// Calls `process_events` and then...
    fn sync(&mut self) {
        self.process_events();
        self.update_nodes();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        composed_area::{ComposedAreaKind, ComposedAreaState, ComposedHtml},
        geo::{Pos, Rect, Size},
        id::Id,
        test_platform::TestPlatform,
    };

    use super::*;

    #[test]
    fn it_works() {
        let mut ctx = CompositionContext {
            platform: TestPlatform::new(),
            world: World::new(Rect::from((0.0, 0.0, 100.0, 100.0))),
            pointer_pos: Pos::new(0.0, 0.0),
        };

        ctx.world.add(
            Id::root(),
            ComposedArea {
                id: Id::from("child"),
                abs_pos: Pos::new(10.0, 10.0),
                size: Size::new(50.0, 50.0),
                kind: ComposedAreaKind::Canvas,
                state: ComposedAreaState::new(),
            },
        );

        ctx.world.add(
            Id::from("child"),
            ComposedArea {
                id: Id::from("grandchild"),
                abs_pos: Pos::new(5.0, 5.0),
                size: Size::new(25.0, 25.0),
                kind: ComposedAreaKind::Html(ComposedHtml {
                    id: "hello".into(),
                    content: "<div>hello</div>".into(),
                }),
                state: ComposedAreaState::new(),
            },
        );

        ctx.world.get(Id::from("grandchild")).unwrap();

        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::root());
        });

        ctx.platform.move_pointer_to(Pos::new(15.0, 15.0));
        ctx.sync();
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("grandchild"));
        });

        ctx.platform.move_pointer_to(Pos::new(30.0, 30.0));
        ctx.sync();
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("grandchild"));
        });

        ctx.platform.move_pointer_to(Pos::new(31.0, 31.0));
        ctx.sync();
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("child"));
        });

        ctx.platform.move_pointer_to(Pos::new(30.0, 30.0));
        ctx.get_under_attention_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("child"));
        });
        ctx.sync();
        ctx.get_under_attention_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("grandchild"));
        });
    }
}
