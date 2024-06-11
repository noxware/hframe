use crate::{
    composed_area::ComposedArea, platform::Platform, test_platform::TestPlatform, tree::Node,
};

struct Context<P: Platform> {
    tree: Node<ComposedArea>,
    platform: P,
}

#[cfg(test)]
mod tests {
    use crate::{
        composed_area::{ComposedAreaKind, ComposedHtml},
        geo::{Pos, Size},
        id::Id,
    };

    use super::*;

    #[test]
    fn it_works() {
        let ctx = Context {
            tree: Node::new(ComposedArea {
                id: Id::from("root"),
                size: Size::new(100.0, 100.0),
                abs_pos: Pos::new(0.0, 0.0),
                kind: ComposedAreaKind::Canvas,
            })
            .nest(
                Node::new(ComposedArea {
                    id: Id::from("child"),
                    size: Size::new(50.0, 50.0),
                    abs_pos: Pos::new(10.0, 10.0),
                    kind: ComposedAreaKind::Canvas,
                })
                .nest(Node::new(ComposedArea {
                    id: Id::from("grandchild"),
                    size: Size::new(25.0, 25.0),
                    abs_pos: Pos::new(5.0, 5.0),
                    kind: ComposedAreaKind::Html(ComposedHtml {
                        content: "<div>hello</div>".into(),
                    }),
                })),
            ),
            platform: TestPlatform {
                mouse_pos: Pos::new(0.0, 0.0),
            },
        };

        ctx.tree
            .find(|node| node.read(|data| data.value.id == Id::from("grandchild")))
            .unwrap();
    }
}
