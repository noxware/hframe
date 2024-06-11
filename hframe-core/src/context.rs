use crate::{
    composed_area::ComposedArea, platform::Platform, test_platform::TestPlatform, tree::Node,
    world::World,
};

struct Context<P: Platform> {
    world: World,
    platform: P,
}

#[cfg(test)]
mod tests {
    use crate::{
        composed_area::{ComposedAreaKind, ComposedHtml},
        geo::{Pos, Rect, Size},
        id::Id,
    };

    use super::*;

    #[test]
    fn it_works() {
        let mut ctx = Context {
            platform: TestPlatform {
                mouse_pos: Pos::new(0.0, 0.0),
            },
            world: World::new(Rect::from((0.0, 0.0, 100.0, 100.0))),
        };

        ctx.world.add(
            Id::root(),
            ComposedArea {
                id: Id::from("child"),
                size: Size::new(50.0, 50.0),
                abs_pos: Pos::new(10.0, 10.0),
                kind: ComposedAreaKind::Canvas,
            },
        );

        ctx.world.add(
            Id::from("child"),
            ComposedArea {
                id: Id::from("grandchild"),
                size: Size::new(25.0, 25.0),
                abs_pos: Pos::new(5.0, 5.0),
                kind: ComposedAreaKind::Html(ComposedHtml {
                    content: "<div>hello</div>".into(),
                }),
            },
        );

        ctx.world.get(Id::from("grandchild")).unwrap();
    }
}
