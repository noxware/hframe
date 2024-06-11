use crate::{
    composed_area::ComposedArea, platform::Platform, test_platform::TestPlatform, tree::Node,
    world::World,
};

struct Context<P: Platform> {
    world: World,
    platform: P,
}

impl<P: Platform> Context<P> {
    fn get_hovered_area(&self) -> Option<Node<ComposedArea>> {
        let mouse_pos = self.platform.mouse_pos();
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

    fn sync(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        composed_area::{ComposedAreaKind, ComposedHtml},
        geo::{Pos, Rect, Size},
        id::Id,
        platform,
    };

    use super::*;

    #[test]
    fn it_works() {
        let platform = TestPlatform::new();

        let mut ctx = Context {
            platform: platform.clone(),
            world: World::new(Rect::from((0.0, 0.0, 100.0, 100.0))),
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
                    content: "<div>hello</div>".into(),
                }),
            },
        );

        ctx.world.get(Id::from("grandchild")).unwrap();

        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::root());
        });

        platform.set_mouse_pos(Pos::new(15.0, 15.0));
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("grandchild"));
        });

        platform.set_mouse_pos(Pos::new(30.0, 30.0));
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("grandchild"));
        });

        platform.set_mouse_pos(Pos::new(31.0, 31.0));
        ctx.get_hovered_area().unwrap().read(|data| {
            assert_eq!(data.value.id, Id::from("child"));
        });
    }
}
