use egui::Rect;

fn split_two_rects(r1: Rect, r2: Rect) -> Vec<Rect> {
    let intersection = r1.intersect(r2);

    let below = Rect::everything_below(intersection.bottom());
    let above = Rect::everything_above(intersection.top());
    let left = Rect::everything_left_of(intersection.left());
    let right = Rect::everything_right_of(intersection.right());

    let mut result = Vec::new();

    for side in [below, above, left, right] {
        for rect in [r1, r2] {
            if rect.intersects(side) {
                result.push(rect.intersect(side))
            }
        }
    }

    result
}

pub fn disjoint_rects(rects: &Vec<Rect>) -> Vec<Rect> {
    log::info!("disjoint_rects: {:?}", rects.len());
    let mut maybe_split_once: Option<Vec<Rect>> = None;
    'outer: for rect in rects.iter().copied() {
        for other_rect in rects.iter().copied().filter(|r| *r != rect) {
            if rect.intersects(other_rect) {
                let split = split_two_rects(rect, other_rect);
                maybe_split_once = Some(
                    rects
                        .iter()
                        .copied()
                        .filter(|r| *r != rect && *r != other_rect)
                        .chain(split)
                        .collect(),
                );
                break 'outer;
            }
        }
    }

    match maybe_split_once {
        Some(rects) => disjoint_rects(&rects),
        None => rects.clone(),
    }
}
