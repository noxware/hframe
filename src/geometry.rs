use egui::emath::Numeric;
use geo::BooleanOps;
use geo::{coord, MultiPolygon, Polygon, Rect};

pub fn egui_rect_to_geo_rect(rect: egui::Rect) -> Rect<f64> {
    Rect::new(
        coord! { x: rect.min.x.to_f64(), y: rect.min.y.to_f64()},
        coord! { x: rect.max.x.to_f64(), y: rect.max.y.to_f64()},
    )
}

pub fn rects_to_union_polygons(rects: &[Rect<f64>]) -> MultiPolygon<f64> {
    rects
        .iter()
        .copied()
        .map(|r| MultiPolygon::new(vec![r.to_polygon()]))
        .reduce(|p1, p2| p1.union(&p2))
        .unwrap_or(MultiPolygon::new(vec![]))
}

pub fn polygon_to_svg_path_counter_clockwise(polygon: &Polygon<f64>) -> String {
    let mut path = String::new();
    for (i, coord) in polygon.exterior().0.iter().rev().enumerate() {
        if i == 0 {
            path.push_str("M");
        } else {
            path.push_str("L");
        }
        path.push_str(&format!("{},{}", coord.x, coord.y));
    }
    path.push_str("Z");
    path
}
