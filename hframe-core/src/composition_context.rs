use crate::{
    composed_area::ComposedArea,
    composition_strategy::CompositionStrategy,
    geo::Pos,
    platform::{Platform, PlatformEvent},
    tree::Node,
    world::World,
};

/// Hold the state of the `hframe` world, the platform abstraction for reading events,
/// provides queries for different info, etc.
pub struct CompositionContext<P: Platform> {
    pub world: World,
    platform: P,
    pointer_pos: Pos,
    // Preserved the possibility of switching strategies at runtime although I probably won't use it.
    strategy: Box<dyn CompositionStrategy<P>>,
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

    /// Calls `process_events` and then...
    fn sync(&mut self) {
        self.process_events();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        composed_area::{ComposedAreaKind, ComposedHtml},
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
    }
}
